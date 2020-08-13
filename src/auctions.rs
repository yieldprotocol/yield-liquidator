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

use tracing::{debug, debug_span, error, info};

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

        // TODO: If auction start time > current time and we still cannot buy it,
        // then it means we cannot ever buy it

        let client = self.uniswap.client();

        // get the current gas price
        let gas_price = client.get_gas_price().await?;

        let balance_before = client.get_balance(client.address(), None).await?;

        for opportunity in liquidations {
            let debt = opportunity.debt;
            let user = opportunity.user;
            let timestamp = opportunity.started;
            let span = debug_span!("buying", user = ?user, auction_start = %timestamp);
            let _enter = span.enter();
            // enter span for {user, started}
            let vault = self.liquidations.vaults(user).call().await?;
            if vault.1 == 0 {
                // trace!("0 debt, skipping");
                continue;
            }

            // Borrows DAI from Uniswap and sends it to the `flashloan` address.
            // It is expected that the flashloan address is a contract which
            // calls the Liquidation contract's `buy` function by sending the DAI
            // and getting the ETH at a discount.
            // e.g. if you borrowed 100 DAI from Uniswap, when the price is $200
            // per ETH, Uniswap would expect that you'd return it 0.5 ETH at the end
            // of the call. The 100 DAI would get sent to the Yield contract in return
            // for, say, 0.52 ETH. The 0.5 ETH would then get sent back to Uniswap
            // and the 0.02 ETH would get pocketed as profit. Careful with gas costs!
            // This call should _not_ be made if gas costs would reduce profit to <0.
            let args = abi::encode(&(user, 0).into_tokens());
            let try_estimate_gas = self
                .uniswap
                .swap(debt, 0.into(), self.flashloan, args)
                .estimate_gas()
                .await;

            let estimated_gas = match try_estimate_gas {
                Ok(gas) => gas,
                Err(err) => {
                    let price = self.liquidations.price(user).call().await?;
                    let required = debt / price;
                    let err = err.to_string();
                    if err.contains("NOT_ENOUGH_PROFIT") {
                        debug!(debt = %debt, required = %required, price = %price, "Auction not yet profitable. Please wait for the price to drop.");
                    } else if err.contains("Below dust") {
                        debug!("Proceeds are below the dust. Please wait for the price to drop.");
                    } else {
                        error!("Error: {}", err);
                    }

                    continue;
                }
            };

            // min profit should be more than 2x eth paid for gas
            let min_profit_eth = estimated_gas * gas_price * U256::from(2);
            // debug!("minimum profit: {}", min_profit_eth);
            let args = abi::encode(&(user, min_profit_eth).into_tokens());
            let tx_hash = self
                .uniswap
                .swap(debt, 0.into(), self.flashloan, args)
                .block(BlockNumber::Pending)
                .send()
                .await?;
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
