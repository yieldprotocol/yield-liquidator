//! Borrowers / Users
//!
//! This module is responsible for keeping track of the users that have open
//! positions and observing their debt healthiness.
use crate::{bindings::Controller, Result, WETH};

use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, debug_span};

#[derive(Clone)]
pub struct Borrowers<M> {
    /// The controller smart contract
    pub controller: Controller<M>,

    /// Mapping of the addresses that have taken loans from the system and might
    /// be susceptible to liquidations
    pub borrowers: HashMap<Address, Borrower>,

    /// We use multicall to batch together calls and have reduced stress on
    /// our RPC endpoint
    multicall: Multicall<M>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// A user's details
pub struct Borrower {
    /// Is the position collateralized? Produced by calling `isCollateralized`
    /// on the controller
    pub is_collateralized: bool,

    /// The user's currently posted ETH collateral. Produced by calling `posted`
    /// on the controller
    pub posted_collateral: U256,

    /// The user's total DAI debt. Produced by calling `totalDebtDai`
    /// on the controller
    pub debt: U256,

    /// The maximum YDAI amount a user can borrow. Produced by calling `powerOf`
    /// on the controller
    pub max_borrowing_power: U256,
}

impl<M: Middleware> Borrowers<M> {
    /// Constructor
    pub async fn new(
        controller: Address,
        multicall: Option<Address>,
        client: Arc<M>,
        borrowers: HashMap<Address, Borrower>,
    ) -> Self {
        let multicall = Multicall::new(client.clone(), multicall)
            .await
            .expect("could not initialize multicall");
        Borrowers {
            controller: Controller::new(controller, client),
            borrowers,
            multicall,
        }
    }

    /// Gets any new borrowers which may have joined the system since we last
    /// made this call and then proceeds to get the latest account details for
    /// each user
    pub async fn update_borrowers(&mut self, from_block: U64, to_block: U64) -> Result<(), M> {
        let span = debug_span!("monitoring");
        let _enter = span.enter();

        // get the new users
        // TODO: Improve this logic to be more optimized
        let new_users = self
            .controller
            .borrowed_filter()
            .from_block(from_block)
            .to_block(to_block)
            .query()
            .await?
            .into_iter()
            .map(|log| log.user)
            .collect::<Vec<_>>();

        let all_users = crate::merge(new_users, &self.borrowers);

        // update all the accounts' details
        for user in all_users {
            let details = self.get_borrower(user).await?;
            if self.borrowers.insert(user, details.clone()).is_none() {
                debug!(new_borrower = ?user, collateral_eth = %details.posted_collateral, debt_dai = %details.debt);
            }
        }

        Ok(())
    }

    /// Updates the user's details by calling:
    /// 1. powerOf
    /// 2. isCollateralized
    /// 3. posted
    /// 4. totalDebtDai
    pub async fn get_borrower(&mut self, user: Address) -> Result<Borrower, M> {
        let power = self.controller.power_of(WETH, user);
        let is_collateralized = self.controller.is_collateralized(WETH, user);
        let posted_collateral = self.controller.posted(WETH, user);
        let debt = self.controller.total_debt_dai(WETH, user);

        // batch the calls together
        let multicall = self
            .multicall
            .clear_calls()
            .add_call(is_collateralized)
            .add_call(posted_collateral)
            .add_call(debt)
            .add_call(power);
        let (is_collateralized, posted_collateral, debt, max_borrowing_power) =
            multicall.call().await?;

        Ok(Borrower {
            is_collateralized,
            posted_collateral,
            debt,
            max_borrowing_power,
        })
    }
}
