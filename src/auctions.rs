//! Auctions Module
//!
//! This module is responsible for triggering and participating in a Vault's
//! dutch auction
use crate::{bindings::Liquidations, positions::Details};
use ethers::prelude::*;

// Auxiliary functions around the typesafe auto-generated bindings
impl Liquidations<Http, Wallet> {
    /// Sends a bid for any of the liquidation auctions.
    // NB: This is the default software. As a result, we do not care to provide
    // optimized versions which are guaranteed to win the auction. If we have submitted
    // a bid and it loses, this means that somebody else will complete the auction -
    // which is great!
    pub async fn buy_opportunities(&self) -> anyhow::Result<()> {
        let addr = self.client().address();
        let liquidations = self.liquidation_filter().from_block(0).query().await?;

        // TODO: Make this take into account DAI
        for opportunity in liquidations {
            self.buy(addr, addr, opportunity.user, opportunity.debt)
                .send()
                .await?;
        }

        Ok(())
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the
    /// controller
    pub async fn trigger_liquidations(
        &self,
        borrowers: impl Iterator<Item = (&Address, &Details)>,
    ) -> anyhow::Result<()> {
        let client = self.client();

        for (user, details) in borrowers {
            if !details.is_collateralized {
                println!(
                    "Found undercollateralized position: {:?} -> {:?}. Claiming fee.",
                    user, details
                );
                let tx_hash = self.liquidate(*user).send().await?;

                // wait for it to be confirmed (TODO: Add number of confs here)
                client.pending_transaction(tx_hash).confirmations(0).await?;
            } else {
                println!("user is collateralized {:?} -> {:?}", user, details);
            }
        }

        Ok(())
    }
}
