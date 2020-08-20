use ethers::prelude::*;
use yield_liquidator::keeper::Keeper;

use gumdrop::Options;
use std::{path::PathBuf, sync::Arc, time::Duration};
use tracing::info;
use tracing_subscriber::{filter::EnvFilter, fmt::Subscriber};

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

    #[options(help = "the file to be used for persistence", default = "data.json")]
    file: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let opts = Opts::parse_args_default_or_exit();

    let ws = Ws::connect(opts.url.clone()).await?;
    let provider = Provider::new(ws);
    // let provider = Provider::<Http>::try_from(opts.url.clone())?;
    let wallet: Wallet = opts.private_key.parse()?;
    let client = wallet
        .connect(provider)
        .interval(Duration::from_millis(opts.interval));
    let client = Arc::new(client);
    info!("Starting Yield Liquidator.");
    info!("Node: {}", opts.url);
    info!("Profits will be sent to {:?}", client.address());
    info!("Controller: {:?}", opts.controller);
    info!("Liquidations: {:?}", opts.liquidations);
    info!("Uniswap: {:?}", opts.uniswap);
    info!("FlashLiquidator {:?}", opts.flashloan);
    info!("Persistent data will be stored at: {:?}", opts.file);

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&opts.file)
        .unwrap();
    let state = serde_json::from_reader(&file).unwrap_or_default();

    let mut keeper = Keeper::new(
        client,
        opts.controller,
        opts.liquidations,
        opts.uniswap,
        opts.flashloan,
        state,
    )
    .await?;

    keeper.run(opts.file).await?;

    Ok(())
}
