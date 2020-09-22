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
pub struct Liquidator<P> {
    liquidations: Liquidations<P, Wallet>,
    uniswap: Uniswap<P, Wallet>,
    flashloan: Address,

    /// The currently active auctions
    pub auctions: HashMap<Address, Auction>,

    /// We use multicall to batch together calls and have reduced stress on
    /// our RPC endpoint
    multicall: Multicall<P, Wallet>,

    /// The minimum profit to be extracted per liquidation
    min_profit: U256,

    pending_liquidations: HashMap<Address, PendingTransaction>,
    pending_auctions: HashMap<Address, PendingTransaction>,
}

/// Tx / Hash/ Submitted at time
type PendingTransaction = (TransactionRequest, TxHash, Instant);

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

impl<P: JsonRpcClient> Liquidator<P> {
    /// Constructor
    pub async fn new(
        liquidations: Address,
        uniswap: Address,
        flashloan: Address,
        multicall: Option<Address>,
        min_profit: U256,
        client: Arc<Client<P, Wallet>>,
        auctions: HashMap<Address, Auction>,
    ) -> Self {
        let multicall = Multicall::new(client.clone(), multicall)
            .await
            .expect("could not initialize multicall");

        Self {
            liquidations: Liquidations::new(liquidations, client.clone()),
            uniswap: Uniswap::new(uniswap, client.clone()),
            flashloan,
            multicall,
            min_profit,
            auctions,

            pending_liquidations: HashMap::new(),
            pending_auctions: HashMap::new(),
        }
    }

    /// Checks if any transactions which have been submitted are mined, and removes
    /// them if they were successful.
    pub async fn remove_confirmed(&mut self) -> Result<()> {
        let client = self.liquidations.client();
        for (addr, tx_hash) in self.pending_auctions.clone().into_iter() {
            let receipt = client.get_transaction_receipt(tx_hash).await?;
            if let Some(receipt) = receipt {
                self.pending_auctions.remove(&addr);
                let status = if receipt.status == Some(1.into()) {
                    "success"
                } else {
                    "fail"
                };
                trace!(tx_hash = ?tx_hash, user = ?addr, status = status, "bid confirmed");
            }
        }

        for (addr, tx_hash) in self.pending_liquidations.clone().into_iter() {
            let receipt = client.get_transaction_receipt(tx_hash).await?;
            if let Some(receipt) = receipt {
                self.pending_liquidations.remove(&addr);
                let status = if receipt.status == Some(1.into()) {
                    "success"
                } else {
                    "fail"
                };
                trace!(tx_hash = ?tx_hash, user = ?addr, status = status, "liquidation confirmed");
            }
        }

        Ok(())
    }

    /// Sends a bid for any of the liquidation auctions.
    pub async fn buy_opportunities(
        &mut self,
        from_block: U64,
        to_block: U64,
        gas_price: U256,
    ) -> Result<()> {
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

        for user in all_users {
            self.buy(user, Instant::now(), gas_price).await?;
        }

        Ok(())
    }

    /// Tries to buy the collateral associated with a user's liquidation auction
    /// via a flashloan funded by Uniswap's DAI/WETH pair.
    async fn buy(&mut self, user: Address, now: Instant, gas_price: U256) -> Result<()> {
        // only iterate over users that do not have active auctions
        if let Some(pending_tx) = self.pending_auctions.get(&user) {
            trace!(tx_hash = ?pending_tx.1, user = ?user, "bid not confirmed yet");
            return Ok(());
        }

        // Get the vault's info
        let vault = self.get_vault(user).await?;
        // Skip auctions which do not have any outstanding debt
        if vault.debt == 0 {
            return Ok(());
        }

        if self.auctions.insert(user, vault.clone()).is_none() {
            debug!(user = ?user, vault = ?vault, "new auction");
        }
        let span = debug_span!("buying", user = ?user, auction_start = %vault.started, auction_end = %(vault.started + 3600), debt = %vault.debt);
        let _enter = span.enter();

        // Craft the flashloan contract's arguments
        let args = abi::encode(&(user, self.min_profit).into_tokens());

        // Calls Uniswap's `swap` function which will optimistically let us
        // borrow the debt, which will then make a callback to the flashloan
        // contract which will execute the liquidation
        let call = self
            .uniswap
            .swap(vault.debt.into(), 0.into(), self.flashloan, args)
            .gas_price(gas_price)
            .block(BlockNumber::Pending);

        let tx = call.tx.clone();

        match call.send().await {
            Ok(hash) => {
                // record the tx
                trace!(tx_hash = ?hash, "Submitted buy order");
                self.pending_auctions.entry(user).or_insert((tx, hash, now));
            }
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
            }
        };

        Ok(())
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the
    /// controller
    pub async fn trigger_liquidations(
        &mut self,
        borrowers: impl Iterator<Item = (&Address, &Borrower)>,
        gas_price: U256,
    ) -> Result<()> {
        debug!("checking for undercollateralized positions...");

        let now = Instant::now();

        for (user, details) in borrowers {
            // only iterate over users that do not have pending liquidations
            if let Some(pending_tx) = self.pending_liquidations.get(&user) {
                trace!(tx_hash = ?pending_tx.1, user = ?user, "liquidation not confirmed yet");
                continue;
            }

            if !details.is_collateralized {
                info!(
                    user = ?user,
                    debt_dai = %details.debt,
                    max_debt_dai = %details.max_borrowing_power,
                    "found undercollateralized user. triggering liquidation",
                );

                // Send the tx and track it
                let call = self.liquidations.liquidate(*user).gas_price(gas_price);
                let tx = call.tx.clone();
                let tx_hash = call.send().await?;
                trace!(tx_hash = ?tx_hash, user = ?user, "Submitted liquidation");
                self.pending_liquidations
                    .entry(*user)
                    .or_insert((tx, tx_hash, now));
            }
        }

        Ok(())
    }

    async fn get_vault(&mut self, user: Address) -> Result<Auction> {
        let vault = self.liquidations.vaults(user);
        let timestamp = self.liquidations.liquidations(user);

        let multicall = self
            .multicall
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
