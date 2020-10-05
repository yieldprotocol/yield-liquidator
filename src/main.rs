use ethers::prelude::*;
use yield_liquidator::{escalator::GeometricGasPrice, keeper::Keeper};

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

    #[options(help = "the block to start watching from")]
    start_block: Option<u64>,
}

#[derive(Deserialize)]
struct Config {
    #[serde(rename = "Controller")]
    controller: Address,
    #[serde(rename = "Liquidations")]
    liquidations: Address,
    #[serde(rename = "Uniswap")]
    uniswap: Address,
    #[serde(rename = "Flash")]
    flashloan: Address,
    #[serde(rename = "Multicall")]
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

async fn run<P: JsonRpcClient + 'static>(opts: Opts, provider: Provider<P>) -> anyhow::Result<()> {
    info!("Starting Yield Liquidator.");
    let provider = provider.interval(Duration::from_millis(opts.interval));
    let wallet: LocalWallet = std::fs::read_to_string(opts.private_key)?.parse()?;
    let address = wallet.address();
    let client = Client::new(provider, wallet);
    let client = NonceManager::new(client, address);
    let client = Arc::new(client);
    info!("Profits will be sent to {:?}", address);

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

    let mut gas_escalator = GeometricGasPrice::new();
    gas_escalator.coefficient = 1.12501;
    gas_escalator.every_secs = 5; // TODO: Make this be 90s
    gas_escalator.max_price = Some(U256::from(5000 * 1e9 as u64)); // 5k gwei

    let mut keeper = Keeper::new(
        client,
        cfg.controller,
        cfg.liquidations,
        cfg.uniswap,
        cfg.flashloan,
        cfg.multicall,
        opts.min_profit,
        gas_escalator,
        state,
    )
    .await?;

    keeper.run(opts.file, opts.start_block).await?;

    Ok(())
}
