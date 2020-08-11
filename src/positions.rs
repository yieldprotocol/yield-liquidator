//! Positions / Users
//!
//! This module is responsible for keeping track of the users that have open
//! positions and observing their debt healthiness.
use crate::{bindings::Controller, WETH};

use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufWriter};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// A user's details
pub struct Details {
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

#[derive(Clone)]
pub struct Positions {
    /// The controller smart contract
    pub controller: Controller<Http, Wallet>,
    /// Mapping of the addresses that have taken loans from the system and might
    /// be susceptible to liquidations
    pub borrowers: HashMap<Address, Details>,
    /// The last block we have observed
    pub last_block: U64,

    multicall: Multicall<Http, Wallet>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
// Data that gets dumped to JSON
struct Data {
    borrowers: HashMap<Address, Details>,
    last_block: U64,
}

const FNAME: &str = "data.json";

impl Positions {
    /// Constructor
    pub fn new(
        controller: Controller<Http, Wallet>,
        multicall: Multicall<Http, Wallet>,
    ) -> anyhow::Result<Self> {
        // TODO: Improve I/O logic
        let file = std::fs::read_to_string(FNAME)?;
        let data: Data = serde_json::from_str(&file)?;
        println!("Imported positions: {:#?}", data);
        Ok(Positions {
            controller,
            borrowers: data.borrowers,
            last_block: data.last_block,
            multicall,
        })
    }

    /// Gets any new borrowers which may have joined the system since we last
    /// made this call and then proceeds to get the latest account details for
    /// each user
    pub async fn update_positions(&mut self) -> anyhow::Result<()> {
        // get latest block
        let current_block = self.controller.client().get_block_number().await?;

        // get the new users
        let new_users = self
            .controller
            .borrowed_filter()
            .from_block(self.last_block)
            .to_block(current_block)
            .query()
            .await?
            .into_iter()
            .map(|log| log.user)
            .collect::<Vec<_>>();

        // combine them with the old users
        let old_users = self.borrowers.keys().cloned().collect::<Vec<_>>();
        let mut all_users = [new_users, old_users].concat();

        // remove any duplicates
        all_users.sort_unstable();
        all_users.dedup();

        // update all the accounts' details
        for user in all_users {
            let details = self.update_account_details(user).await?;
            if self.borrowers.insert(user, details.clone()).is_none() {
                println!("New borrower detected: {:?} -> {:?}", user, details);
            }
        }

        // update last block
        self.last_block = current_block;

        // TODO: Instead of re-serializing everything on top, just append the new
        // borrowers and update the last block
        let file = BufWriter::new(File::create(FNAME).unwrap());
        let data = Data {
            borrowers: self.borrowers.clone(),
            last_block: self.last_block,
        };
        serde_json::to_writer(file, &data)?;

        Ok(())
    }

    /// Updates the user's details by calling:
    /// 1. powerOf
    /// 2. isCollateralized
    /// 3. posted
    /// 4. totalDebtDai
    pub async fn update_account_details(&self, user: Address) -> anyhow::Result<Details> {
        let power = self.controller.power_of(WETH, user);
        let is_collateralized = self.controller.is_collateralized(WETH, user);
        let posted_collateral = self.controller.posted(WETH, user);
        let debt = self.controller.total_debt_dai(WETH, user);

        // batch the calls together
        let multicall = self
            .multicall
            .clone()
            .clear_calls()
            .add_call(is_collateralized)
            .add_call(posted_collateral)
            .add_call(debt)
            .add_call(power);
        let (is_collateralized, posted_collateral, debt, max_borrowing_power) =
            multicall.call().await?;

        Ok(Details {
            is_collateralized,
            posted_collateral,
            debt,
            max_borrowing_power,
        })
    }
}
