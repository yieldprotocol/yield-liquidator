use ethers::prelude::*;
use yield_liquidator::keeper::Keeper;

use gumdrop::Options;
use serde::Deserialize;
use std::{convert::TryFrom, path::PathBuf, sync::Arc, time::Duration};
use tracing::info;
use tracing_subscriber::{filter::EnvFilter, fmt::Subscriber};

// CLI Options
#[derive(Debug, Options, Clone)]
struct Opts {
    help: bool,

    #[options(help = "path to json file with the contract addresses")]
    config: PathBuf,

    #[options(
        help = "the Ethereum node endpoint (HTTP or WS)",
        default = "http://localhost:8545"
    )]
    url: String,

    #[options(help = "path to your private key")]
    private_key: PathBuf,

    #[options(help = "polling interval (ms)", default = "1000")]
    interval: u64,

    #[options(help = "the file to be used for persistence", default = "data.json")]
    file: PathBuf,

    #[options(help = "the minimum profit per liquidation", default = "0")]
    min_profit: U256,
}

#[derive(Deserialize)]
struct Config {
    controller: Address,
    liquidations: Address,
    uniswap: Address,
    flashloan: Address,
    multicall: Option<Address>,
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
    info!("Starting Yield Liquidator.");
    let wallet: Wallet = std::fs::read_to_string(opts.private_key)?.parse()?;
    let client = wallet
        .connect(provider)
        .interval(Duration::from_millis(opts.interval))
        // enable the nonce-manager so that we can send multiple transactions in a row
        // without waiting for them to be included in the mempool
        .with_nonce_manager();
    let client = Arc::new(client);
    info!("Profits will be sent to {:?}", client.address());

    info!("Node: {}", opts.url);

    let cfg: Config = serde_json::from_reader(std::fs::File::open(opts.config)?)?;
    info!("Controller: {:?}", cfg.controller);
    info!("Liquidations: {:?}", cfg.liquidations);
    info!("Uniswap: {:?}", cfg.uniswap);
    info!("Multicall: {:?}", cfg.multicall);
    info!("FlashLiquidator {:?}", cfg.flashloan);
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
        cfg.controller,
        cfg.liquidations,
        cfg.uniswap,
        cfg.flashloan,
        cfg.multicall,
        opts.min_profit,
        state,
    )
    .await?;

    keeper.run(opts.file).await?;

    Ok(())
}
