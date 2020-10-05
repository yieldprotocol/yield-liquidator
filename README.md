# Yield Protocol Liquidator

Liquidates undercollateralized fyDAI-ETH positions using Uniswap V2 as a capital source.

This liquidator altruistically calls the `Liquidations.liquidate` function for any
position that is underwater, trigerring an auction for that position. It then tries
to participate in the auction by flashloaning funds from Uniswap, if there's enough
profit to be made.

## CLI

```
Usage: ./yield-liquidator [OPTIONS]

Optional arguments:
  -h, --help
  -c, --config CONFIG      path to json file with the contract addresses
  -u, --url URL            the Ethereum node endpoint (HTTP or WS) (default: http://localhost:8545)
  -p, --private-key PRIVATE-KEY
                           path to your private key
  -i, --interval INTERVAL  polling interval (ms) (default: 1000)
  -f, --file FILE          the file to be used for persistence (default: data.json)
  -m, --min-profit MIN-PROFIT
                           the minimum profit per liquidation (default: 0)
```

Your contracts' `--config` file should be in the following format where `Uniswap` is the
UniswapV2 WETH/DAI pair and `Flash` is the [Flashloan](./Flash.sol) contract.

```
{
   "Controller" : "0xd160C973a098608e2D7d6E43C64Eee48766800D1",
   "Liquidations" : "0xbC0200F0AAD7C1c0bBB1CC7885E1e796DFFac3e0",
   "Uniswap": "0xbC0200F0AAD7C1c0bBB1CC7885E1e796DFFac3e0",
   "Flash": "0xbC0200F0AAD7C1c0bBB1CC7885E1e796DFFac3e0"
}
```

The `--private-key` _must not_ have a `0x` prefix. Set the `interval` to 15s for mainnet.

## Building and Running

```
# Build in release mode
cargo build --release

# Run it with 
./target/release/yield-liquidator \
    --config ./addrs.json \
    --private-key ./private_key \
    --url http://localhost:8545 \
    --interval 7000 \
    --file state.json \
```

## How it Works

On each block:
1. Bumps the gas price of all of our pending transactions
2. Updates our dataset of borrowers debt health & liquidation auctions with the new block's data
3. Trigger the auction for any undercollateralized borrowers
4. Try participating in any auctions which are worth buying

Take this liquidator for a spin by [running it in a test environment](TESTNET.md).
