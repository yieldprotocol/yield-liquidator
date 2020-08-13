use yield_liquidator::{
    auctions::Liquidator,
    bindings::{Controller, Liquidations, UniswapV2Pair as Uniswap},
    positions::Positions,
};

use tracing::debug_span;
use tracing_subscriber::{filter::EnvFilter, fmt::Subscriber};

use ethers::prelude::*;
use gumdrop::Options;
use std::{convert::TryFrom, time::Duration};

// CLI Options
#[derive(Debug, Options, Clone)]
struct Opts {
    help: bool,

    #[options(
        help = "the Yield controller's address",
        default = "595D20A216072a4634db3Ec51736b9B0402b1C86"
    )]
    controller: Address,

    #[options(
        help = "the Yield liquidation's address",
        default = "00447Fe0075094C24fd5DFec3262b6e13eD2913D"
    )]
    liquidations: Address,

    #[options(
        help = "the DAI/WETH Uniswap V2 pair",
        default = "e831dcd2aCb881Ca4d75c08772551FCCdED3FAd6"
    )]
    uniswap: Address,

    #[options(
        help = "the address of your flashloan contract",
        default = "2591D67697C7d9d80bE69081cB746Ce1cDC1387b"
    )]
    flashloan: Address,

    #[options(
        help = "the Ethereum node HTTP endpoint",
        default = "http://localhost:8545"
    )]
    url: String,

    #[options(
        help = "your private key",
        default = "5d50201676371a54c2cace1b26c114cded7c938ec296d9cbee697ab515bf1ff7"
    )]
    private_key: String,

    #[options(help = "polling interval (ms)", default = "1000")]
    interval: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let opts = Opts::parse_args_default_or_exit();

    // 1. Instantiate the client
    let provider = Provider::<Http>::try_from(opts.url)?;
    let wallet: Wallet = opts.private_key.parse()?;
    let client = wallet
        .connect(provider)
        .interval(Duration::from_millis(opts.interval));

    // 2. Instantiate the data watcher
    let mut positions = {
        let controller = Controller::new(opts.controller, client.clone());

        let multicall = Multicall::new(
            client.clone(),
            // TODO: Remove this
            Some("05Bc42F1fd5A92b896a529FDE14414Faf30da482".parse()?),
        )
        .await?;

        Positions::new(controller, multicall)?
    };

    // 3. Instantiate the liquidator
    let liquidator = {
        let liquidations = Liquidations::new(opts.liquidations, client.clone());
        let uniswap = Uniswap::new(opts.uniswap, client.clone());

        Liquidator::new(liquidations, uniswap, opts.flashloan)
    };

    // setup the per-block watcher
    let mut on_block = client.watch_blocks().await?.stream();
    while let Some(_) = on_block.next().await {
        let block_number = client.get_block_number().await?;
        let span = debug_span!("eloop", block = %block_number);
        let _enter = span.enter();

        // 1. update our dataset with the new block's data
        positions.update_positions(block_number).await?;

        // 2. trigger the auction for any undercollateralized positions
        liquidator
            .trigger_liquidations(positions.borrowers.iter())
            .await?;

        // 3. try buying the ones which are worth buying
        liquidator.buy_opportunities().await?;
    }

    Ok(())
}
