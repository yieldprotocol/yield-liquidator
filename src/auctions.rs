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
        dbg!(&liquidations);

        for opportunity in liquidations {
            let debt = opportunity.debt;
            let user = abi::encode(&opportunity.user.into_tokens());
            dbg!(&user);
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
            let tx_hash = self
                .uniswap
                .swap(debt, 0.into(), self.flashloan, user)
                .send()
                .await?;
            // Wait for the tx to be confirmed...
            let _receipt = self.uniswap.client().pending_transaction(tx_hash).await?;
        }

        Ok(())
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the
    /// controller
    pub async fn trigger_liquidations(
        &self,
        borrowers: impl Iterator<Item = (&Address, &Details)>,
    ) -> anyhow::Result<()> {
        let client = self.liquidations.client();

        for (user, details) in borrowers {
            if !details.is_collateralized {
                println!(
                    "Found undercollateralized position: {:?} -> {:?}. Claiming fee.",
                    user, details
                );
                let tx_hash = self.liquidations.liquidate(*user).send().await?;

                // wait for it to be confirmed (TODO: Add number of confs here)
                client.pending_transaction(tx_hash).confirmations(0).await?;
            } else {
                println!("user is collateralized {:?} -> {:?}", user, details);
            }
        }

        Ok(())
    }
}
