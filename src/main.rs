use yield_liquidator::{
    auctions::Liquidator,
    bindings::{Controller, Liquidations, UniswapV2Pair as Uniswap},
    positions::Positions,
};

use ethers::prelude::*;
use std::{convert::TryFrom, time::Duration};

// TODO: Change this for mainnet/testnets etc.
const INTERVAL: Duration = Duration::from_millis(1000);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // instantiate the provider
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let wallet: Wallet =
        "5d50201676371a54c2cace1b26c114cded7c938ec296d9cbee697ab515bf1ff7".parse()?;
    let client = wallet.connect(provider).interval(INTERVAL);

    let controller: Address = "595D20A216072a4634db3Ec51736b9B0402b1C86".parse()?;
    let controller = Controller::new(controller, client.clone());

    let liquidations: Address = "00447Fe0075094C24fd5DFec3262b6e13eD2913D".parse()?;
    let liquidations = Liquidations::new(liquidations, client.clone());

    let uniswap: Address = "22B831B023A67d2A7370d1d69Fb711e14C375458".parse()?;
    let uniswap = Uniswap::new(uniswap, client.clone());

    // TODO: Replace this with the actual address of the flashloaner
    let flashloan: Address = "22B831B023A67d2A7370d1d69Fb711e14C375458".parse()?;

    let liquidator = Liquidator::new(liquidations, uniswap, flashloan);

    let multicall = Multicall::new(
        client.clone(),
        Some("05Bc42F1fd5A92b896a529FDE14414Faf30da482".parse()?),
    )
    .await?;

    // instantiate the accounts watcher
    let mut positions = Positions::new(controller, multicall)?;

    // setup the per-block watcher
    let mut on_block = client.watch_blocks().await?.stream();

    while let Some(blk) = on_block.next().await {
        dbg!(blk);
        // 1. update our dataset with the new block's data
        positions.update_positions().await?;

        // 2. trigger the auction for any undercollateralized positions
        // liquidations
        //     .trigger_liquidations(positions.borrowers.iter())
        //     .await?;

        // 3. try buying the ones which are worth buying
        liquidator.buy_opportunities().await?;
    }

    Ok(())
}
