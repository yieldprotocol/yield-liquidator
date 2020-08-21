use ethers::prelude::*;
use yield_liquidator::keeper::Keeper;

use gumdrop::Options;
use std::{convert::TryFrom, path::PathBuf, sync::Arc, time::Duration};
use tracing::info;
use tracing_subscriber::{filter::EnvFilter, fmt::Subscriber};

// CLI Options
#[derive(Debug, Options, Clone)]
struct Opts {
    help: bool,

    #[options(
        help = "the Yield controller's address",
        default = "Cf699af73C25aC785E8Da72F4fA7872c86D43C15"
    )]
    controller: Address,

    #[options(
        help = "the Yield liquidation's address",
        default = "b85F3d294edD2B76128cf918800BB21081f59223"
    )]
    liquidations: Address,

    #[options(
        help = "the DAI/WETH Uniswap V2 pair",
        default = "6025b901C88e5739Cb4112fcf3F27E0264c5BdDe"
    )]
    uniswap: Address,

    #[options(
        help = "the address of your flashloan contract",
        default = "FE35d86cb5b6c0273fAC070Cc40aFeA77574cEF0"
    )]
    flashloan: Address,

    #[options(
        help = "the address of the Multicall contract (optional for any of the deployed testnets)"
    )]
    multicall: Option<Address>,

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

    #[options(help = "the minimum profit per liquidation", default = "0")]
    min_profit: U256,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let opts = Opts::parse_args_default_or_exit();

    if opts.url.starts_with("http") {
        let provider = Provider::<Http>::try_from(opts.url.clone())?;
        run(opts, provider).await?;
    } else {
        let ws = Ws::connect(opts.url.clone()).await?;
        let provider = Provider::new(ws);
        run(opts, provider).await?;
    }

    Ok(())
}

async fn run<P: JsonRpcClient>(opts: Opts, provider: Provider<P>) -> anyhow::Result<()> {
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
    info!("Multicall: {:?}", opts.multicall);
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
        opts.multicall,
        opts.min_profit,
        state,
    )
    .await?;

    keeper.run(opts.file).await?;

    Ok(())
}
