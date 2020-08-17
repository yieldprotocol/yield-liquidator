//! Auctions Module
//!
//! This module is responsible for triggering and participating in a Auction's
//! dutch auction
use crate::{
    bindings::{Liquidations, UniswapV2Pair as Uniswap},
    borrowers::Borrower,
    merge,
};

use anyhow::Result;
use ethers::{
    core::abi::{self, Tokenize},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, debug_span, error, info, trace};

#[derive(Clone)]
pub struct Liquidator<'a> {
    liquidations: Liquidations<Http, Wallet>,
    uniswap: Uniswap<Http, Wallet>,
    flashloan: Address,

    /// The currently active auctions
    pub auctions: HashMap<Address, Auction>,

    /// We use multicall to batch together calls and have reduced stress on
    /// our RPC endpoint
    multicall: &'a Multicall<Http, Wallet>,
}

/// An initiated auction
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Auction {
    /// The start time of the auction
    started: U256,
    /// The debt which can be repaid
    debt: u128,
    /// The collateral which can be seized
    collateral: u128,
}

impl<'a> Liquidator<'a> {
    /// Constructor
    pub fn new(
        liquidations: Address,
        uniswap: Address,
        flashloan: Address,
        client: Arc<Client<Http, Wallet>>,
        multicall: &'a Multicall<Http, Wallet>,
        auctions: HashMap<Address, Auction>,
    ) -> Self {
        Self {
            liquidations: Liquidations::new(liquidations, client.clone()),
            uniswap: Uniswap::new(uniswap, client.clone()),
            flashloan,
            multicall,
            auctions,
        }
    }

    /// Sends a bid for any of the liquidation auctions.
    pub async fn buy_opportunities(&mut self, from_block: U64, to_block: U64) -> Result<()> {
        let all_users = {
            let liquidations = self
                .liquidations
                .liquidation_filter()
                .from_block(from_block)
                .to_block(to_block)
                .query()
                .await?;
            let new_users = liquidations.iter().map(|log| log.user).collect::<Vec<_>>();
            merge(new_users, &self.auctions)
        };

        let address = self.uniswap.client().address();
        let balance_before = self.uniswap.client().get_balance(address, None).await?;
        for user in all_users {
            self.buy(user).await?;
        }
        let balance_after = self.uniswap.client().get_balance(address, None).await?;
        info!(
            "done buying. profit: {} WEI",
            (balance_after - balance_before)
        );

        Ok(())
    }

    /// Tries to buy the collateral associated with a user's liquidation auction
    /// via a flashloan funded by Uniswap's DAI/WETH pair.
    async fn buy(&mut self, user: Address) -> Result<()> {
        let vault = self.get_vault(user).await?;
        let timestamp = vault.started;
        let debt = vault.debt;
        if self.auctions.insert(user, vault.clone()).is_none() {
            debug!(user = ?user, vault = ?vault, "new auction");
        }

        let span = debug_span!("buying", user = ?user, auction_start = %timestamp, auction_end = %(timestamp + 3600), debt = %debt);
        let _enter = span.enter();

        // Skip auctions which do not have any outstanding debt
        if vault.debt == 0 {
            trace!("Skipping 0 debt vault");
            return Ok(());
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
            .swap(debt.into(), 0.into(), self.flashloan, args)
            .block(BlockNumber::Pending)
            .send()
            .await
        {
            Ok(hash) => hash,
            Err(err) => {
                let err = err.to_string();
                if err.contains("NOT_ENOUGH_PROFIT") {
                    let price = self.liquidations.price(user).call().await?;
                    debug!(price = %price, "Auction not yet profitable via Uniswap flash swaps.");
                } else if err.contains("Below dust") {
                    debug!("Proceeds are below the dust limit, ignoring..");
                } else {
                    error!("Error: {}", err);
                }

                return Ok(());
            }
        };

        // Wait for the tx to be confirmed...
        let receipt = self.uniswap.client().pending_transaction(tx_hash).await?;
        debug!("bought. gas used {}", receipt.gas_used.unwrap());

        Ok(())
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the
    /// controller
    pub async fn trigger_liquidations(
        &self,
        borrowers: impl Iterator<Item = (&Address, &Borrower)>,
    ) -> Result<()> {
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

    async fn get_vault(&self, user: Address) -> Result<Auction> {
        let vault = self.liquidations.vaults(user);
        let timestamp = self.liquidations.liquidations(user);

        // batch the calls together
        let multicall = self
            .multicall
            .clone()
            .clear_calls()
            .add_call(vault)
            .add_call(timestamp);
        let (vault, timestamp): ((u128, u128), U256) = multicall.call().await?;

        Ok(Auction {
            started: timestamp,
            collateral: vault.0,
            debt: vault.1,
        })
    }
}
