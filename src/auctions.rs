//! Auctions Module
//!
//! This module is responsible for triggering and participating in a Vault's
//! dutch auction
use crate::{
    bindings::{Liquidations, UniswapV2Pair as Uniswap},
    positions::Details,
};
use ethers::core::abi::{self, Tokenize};
use ethers::prelude::*;

use tracing::{debug, debug_span, error, info, trace};

pub struct Liquidator {
    liquidations: Liquidations<Http, Wallet>,
    uniswap: Uniswap<Http, Wallet>,
    flashloan: Address,
}

// Auxiliary functions around the typesafe auto-generated bindings
impl Liquidator {
    pub fn new(
        liquidations: Liquidations<Http, Wallet>,
        uniswap: Uniswap<Http, Wallet>,
        flashloan: Address,
    ) -> Self {
        Self {
            liquidations,
            uniswap,
            flashloan,
        }
    }

    /// Sends a bid for any of the liquidation auctions.
    // NB: This is the default software. As a result, we do not care to provide
    // optimized versions which are guaranteed to win the auction. If we have submitted
    // a bid and it loses, this means that somebody else will complete the auction -
    // which is great!
    pub async fn buy_opportunities(&self) -> anyhow::Result<()> {
        let liquidations = self
            .liquidations
            .liquidation_filter()
            .from_block(0)
            .query()
            .await?;

        let client = self.uniswap.client();
        let balance_before = client.get_balance(client.address(), None).await?;

        // TODO: Make the smart contract take an array of (users, balances) ->
        // flash loan everything -> make multiple `buy` calls
        for opportunity in liquidations {
            let user = opportunity.user;
            let vault = self.liquidations.vaults(user).call().await?;
            let debt = U256::from(vault.1);
            let timestamp = opportunity.started;
            let span =
                debug_span!("buying", user = ?user, auction_start = %timestamp, auction_end = %(timestamp + 3600), debt = %debt);
            let _enter = span.enter();
            // enter span for {user, started}
            let vault = self.liquidations.vaults(user).call().await?;
            // Skip vaults which do not have any outstanding debt
            if vault.1 == 0 {
                trace!("Skipping 0 debt vault");
                continue;
            }

            // TODO: Should this be done via gas estimation? A minimum 0.1 ETH
            // profit seems good enough.
            let min_profit_eth = U256::from(1e17 as u64);
            let args = abi::encode(&(user, min_profit_eth).into_tokens());

            // Calls Uniswap's `swap` function which will optimistically let us
            // borrow the debt, which will then make a callback to the flashloan
            // contract which will execute the liquidation
            let tx_hash = match self
                .uniswap
                .swap(debt, 0.into(), self.flashloan, args)
                .block(BlockNumber::Pending)
                .send()
                .await
            {
                Ok(hash) => hash,
                Err(err) => {
                    let price = self.liquidations.price(user).call().await?;
                    let err = err.to_string();
                    if err.contains("NOT_ENOUGH_PROFIT") {
                        debug!(price = %price, "Auction not yet profitable. Please wait for the price to drop.");
                    } else if err.contains("Below dust") {
                        debug!("Proceeds are below the dust. Please wait for the price to drop.");
                    } else {
                        error!("Error: {}", err);
                    }

                    continue;
                }
            };

            // Wait for the tx to be confirmed...
            let receipt = client.pending_transaction(tx_hash).await?;
            debug!("bought. gas used {}", receipt.gas_used.unwrap());
        }

        let balance_after = client.get_balance(client.address(), None).await?;
        info!(
            "done buying. profit: {} WEI",
            (balance_after - balance_before)
        );

        Ok(())
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the
    /// controller
    pub async fn trigger_liquidations(
        &self,
        borrowers: impl Iterator<Item = (&Address, &Details)>,
    ) -> anyhow::Result<()> {
        let client = self.liquidations.client();

        debug!("checking for undercollateralized positions...");

        for (user, details) in borrowers {
            if !details.is_collateralized {
                info!(
                    user = ?user,
                    debt_dai = %details.debt,
                    max_debt_dai = %details.max_borrowing_power,
                    "found undercollateralized user. triggering liquidation",
                );
                let tx_hash = self.liquidations.liquidate(*user).send().await?;
                // wait for it to be confirmed (TODO: Add number of confs here)
                client.pending_transaction(tx_hash).confirmations(0).await?;
            }
        }

        Ok(())
    }
}
