# Yield Protocol Liquidator

Liquidates undercollateralized yDAI-ETH positions.

On each new block, it calls the controller's `isCollateralized` method for each user
that has previously opened a position. This is done by checking all `Borrowed` events 
and by compiling a database of addresses with currently open positions.

If `isCollateralized` returns `false`, it proceeds to call the `Liquidations` contract's 
`liquidate` function to claim the liquidation incentive. This can optionally be batched
with the `buy` call to buy as much of the position as possible and liquidate it 
for DAI on 1inch/uniswap.

## CLI Options

- Yield configuration with necessary addresses
- Eth endpoint
- Private key
- start-block
- should always buy? How much should it try to buy?
- minimum amount

# Notes

- Generate corpus of addresses to monitor
- Nonce manager
- Gas price manager
- Frontrunner
- For an address: check if it can be liquidated -> liquidate
- `trait Liquidator { fn monitor(user: Address); fn liquidate(user: Address);}`

# Testing 

- Go to the yDAI directory and copy the `setup_liquidation.js` file to the `scripts/` dir
- Deploy the contracts `npx truffle migrate --reset`
- `cargo run` to launch the liquidator
- Create a vault that can be liquidated `npx truffle exec scripts/setup_liquidation.js`
- Watch the liquidator see that the vault became undercollateralized!
