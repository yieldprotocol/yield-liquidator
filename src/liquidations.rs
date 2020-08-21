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

    pending_liquidations: HashMap<Address, TxHash>,
    pending_auctions: HashMap<Address, TxHash>,
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

impl<P: JsonRpcClient> Liquidator<P> {
    /// Constructor
    pub async fn new(
        liquidations: Address,
        uniswap: Address,
        flashloan: Address,
        min_profit: U256,
        client: Arc<Client<P, Wallet>>,
        auctions: HashMap<Address, Auction>,
    ) -> Self {
        let multicall = Multicall::new(client.clone(), None)
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
            if receipt.status == Some(1.into()) {
                trace!(tx_hash = ?tx_hash, user = ?addr, "bid confirmed");
                self.pending_auctions.remove(&addr);
            }
        }

        for (addr, tx_hash) in self.pending_liquidations.clone().into_iter() {
            let receipt = client.get_transaction_receipt(tx_hash).await?;
            if receipt.status == Some(1.into()) {
                trace!(tx_hash = ?tx_hash, user = ?addr, "liquidation confirmed");
                self.pending_liquidations.remove(&addr);
            }
        }

        Ok(())
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

        if balance_after > balance_before {
            info!(
                "done buying. profit: {} WEI",
                (balance_after - balance_before)
            );
        }

        Ok(())
    }

    /// Tries to buy the collateral associated with a user's liquidation auction
    /// via a flashloan funded by Uniswap's DAI/WETH pair.
    async fn buy(&mut self, user: Address) -> Result<()> {
        // only iterate over users that do not have active auctions
        if let Some(tx_hash) = self.pending_auctions.get(&user) {
            trace!(tx_hash = ?tx_hash, user = ?user, "bid not confirmed yet");
            return Ok(());
        }

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
            // trace!("Vault liquidated");
            return Ok(());
        }

        let args = abi::encode(&(user, self.min_profit).into_tokens());

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
            Ok(hash) => {
                // record the tx
                trace!(tx_hash = ?hash, "Submitted buy order");
                self.pending_auctions.entry(user).or_insert(hash);
                hash
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
        &mut self,
        borrowers: impl Iterator<Item = (&Address, &Borrower)>,
    ) -> Result<()> {
        debug!("checking for undercollateralized positions...");

        for (user, details) in borrowers {
            // only iterate over users that do not have pending liquidations
            if let Some(tx_hash) = self.pending_liquidations.get(&user) {
                trace!(tx_hash = ?tx_hash, user = ?user, "liquidation not confirmed yet");
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
                let tx_hash = self.liquidations.liquidate(*user).send().await?;
                trace!(tx_hash = ?tx_hash, user = ?user, "Submitted liquidation");
                self.pending_liquidations.entry(*user).or_insert(tx_hash);
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
