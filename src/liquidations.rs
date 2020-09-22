//! Auctions Module
//!
//! This module is responsible for triggering and participating in a Auction's
//! dutch auction
use crate::{
    bindings::{Liquidations, UniswapV2Pair as Uniswap},
    borrowers::Borrower,
    escalator::GeometricGasPrice,
    merge,
};

use anyhow::Result;
use ethers::{
    core::abi::{self, Tokenize},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, sync::Arc, time::Instant};
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
    gas_escalator: GeometricGasPrice,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
enum TxType {
    Auction,
    Liquidation,
}

impl fmt::Display for TxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            TxType::Auction => "auction",
            TxType::Liquidation => "liquidation",
        };
        write!(f, "{}", string)
    }
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
        gas_escalator: GeometricGasPrice,
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
            gas_escalator,
        }
    }

    /// Checks if any transactions which have been submitted are mined, removes
    /// them if they were successful, otherwise bumps their gas price
    pub async fn remove_or_bump(&mut self) -> Result<()> {
        let now = Instant::now();

        // Check all the pending liquidations
        self.remove_or_bump_inner(now, TxType::Liquidation).await?;

        // Check all the pending auctions
        self.remove_or_bump_inner(now, TxType::Auction).await?;

        Ok(())
    }

    async fn remove_or_bump_inner(&mut self, now: Instant, tx_type: TxType) -> Result<()> {
        let client = self.liquidations.client();

        let pending_txs = match tx_type {
            TxType::Liquidation => &mut self.pending_liquidations,
            TxType::Auction => &mut self.pending_auctions,
        };

        for (addr, pending_tx) in pending_txs.clone().into_iter() {
            debug_assert!(
                pending_tx.0.gas_price.is_some(),
                "gas price must be set in pending txs"
            );

            debug_assert!(
                pending_tx.0.nonce.is_some(),
                "nonce must be set in pending txs"
            );

            // get the receipt and check inclusion, or bump its gas price
            let receipt = client.get_transaction_receipt(pending_tx.1).await?;
            if let Some(receipt) = receipt {
                pending_txs.remove(&addr);
                let status = if receipt.status == Some(1.into()) {
                    "success"
                } else {
                    "fail"
                };
                trace!(tx_hash = ?pending_tx.1, gas_used = %receipt.gas_used.unwrap_or_default(), user = ?addr, status = status, tx_type = %tx_type, "confirmed");
            } else {
                // Get the new gas price based on how much time passed since the
                // tx was last broadcast
                let new_gas_price = self.gas_escalator.get_gas_price(
                    pending_tx.0.gas_price.expect("gas price must be set"),
                    now.duration_since(pending_tx.2).as_secs(),
                );

                let replacement_tx = pending_txs
                    .get_mut(&addr)
                    .expect("tx will always be found since we're iterating over the map");

                // bump the gas price
                replacement_tx.0.gas_price = Some(new_gas_price);

                // rebroadcast (TODO: Can we avoid cloning?)
                replacement_tx.1 = client
                    .send_transaction(replacement_tx.0.clone(), None)
                    .await?;

                // update the tx broadcast time
                replacement_tx.2 = now;
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
