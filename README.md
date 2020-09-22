# Yield Protocol Liquidator

Liquidates undercollateralized yDAI-ETH positions using Uniswap V2 as a capital source.

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

Your contracts config file should be in the following format where `uniswap` is the
UniswapV2 WETH/DAI pair and `flashloan` is the [Flash](./Flash.sol) contract.

```
{
   "controller" : "0xd160C973a098608e2D7d6E43C64Eee48766800D1",
   "liquidations" : "0xbC0200F0AAD7C1c0bBB1CC7885E1e796DFFac3e0",
   "uniswap": "0xbC0200F0AAD7C1c0bBB1CC7885E1e796DFFac3e0",
   "flashloan": "0xbC0200F0AAD7C1c0bBB1CC7885E1e796DFFac3e0"
}
```

## Building and Running

```
cargo build --release
./target/release/yield-liquidator \
    --config ./addrs.json \
    --private-key ./private_key \
    --url wss://kovan.infura.io/ws/v3/fd8b88b56aa84f6da87b60f5441d6778 \
    --interval 7000 \
    --file state.json \
```

## How it Works

On each block:
1. Checks if any of our submitted transactions have been mined
2. Updates our dataset of borrowers debt health & liquidation auctions with the new block's data
3. Trigger the auction for any undercollateralized borrowers
4. Try participating in any auctions which are worth buying


## Testing

### Local Testnet

First we must clone the contracts and install the deps:

```
git clone https://github.com/yieldprotocol/yDai
git checkout liquidator-testing
yarn
```

In one terminal, run ganache: `./scripts/ganache.sh`

In another terminal, deploy the contracts: `npx truffle migrate --reset`

This deploys MakerDAO, Yield, UniswapV2, [Multicall](https://github.com/makerdao/multicall) and the Flashloan contract.

Now we run the liquidator as before, by also including the `multicall` contract's address:

```
{
   "controller" : "0xd160C973a098608e2D7d6E43C64Eee48766800D1",
   "liquidations" : "0xbC0200F0AAD7C1c0bBB1CC7885E1e796DFFac3e0",
   "uniswap": "0xbC0200F0AAD7C1c0bBB1CC7885E1e796DFFac3e0",
   "flashloan": "0xbC0200F0AAD7C1c0bBB1CC7885E1e796DFFac3e0"
   "multicall": "0xbC0200F0AAD7C1c0bBB1CC7885E1e796DFFac3e0"
}
```

Finally, you can create a liquidation opportunity by running `npx truffle exec scripts/create_liquidation_opportunity.js`.

Note: re-creating the liquidation opportunity requires balancing the price on Uniswap
so that you can get a good flash-swap price, by increasing the ratio between its DAI / ETH
reserves.

### Rinkeby

The contracts for Rinkeby have already been deployed. You can interact with them using the helper scripts explained below:

First, we must setup our environment:

```
git clone git@github.com:yieldprotocol/yield-cli.git
yarn
source config
```

The `config` file will set your environment variables to allow you to mint tokens to Uniswap, set MakerDAO prices, and borrow or collateralize on Yield.


Now we'll run the liquidator:

```
RUST_LOG=yield_liquidator=trace cargo run -- \
    -c ${CONTROLLER#"0x"} \
    -l ${LIQUIDATIONS#"0x"} \
    -u ${UNISWAP#"0x"} \
    -f ${FLASH#"0x"} \
    -p $PRIVATE_KEY \
    -i 7000 \
    -F rinkeby.json \
    -U wss://rinkeby.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27 \
    --min-profit 0
```

Finally, you can create a liquidation opportunity by running:

```
./src/set_eth_price.js 200
./src/fund_uniswap.js 0 10000
./src/control.js 1 166
./src/set_eth_price.js 175
```

## Known Bugs
