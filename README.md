# Yield Protocol Liquidator

Liquidates undercollateralized ETH positions.

On each new block, it calls the controller's `isCollateralized` method for each user
that has previously opened a position. This is done by checking for all `Posted`
and `Borrowed` events and by compiling a database of addresses with currently open
positions.

If `isCollateralized` returns `false`, it proceeds to call the `Liquidations` contract's 
`liquidate` function to claim the liquidation incentive. This can optionally be batched
with the `buy` call to buy as much of the position as possible and liquidate it 
for DAI on 1inch/uniswap.


CLI Options

- Yield configuration with necessary addresses
- Eth endpoint
- Private key
- start-block
- should always buy? How much should it try to buy?
- minimum amount

# Notes

- `liquidate` erases the user's debt on the controller

- Generate corpus of addresses to monitor
- Nonce manager
- Gas price manager
- Frontrunner
- For an address: check if it can be liquidated -> liquidate
- `trait Liquidator { fn monitor(user: Address); fn liquidate(user: Address);}`
