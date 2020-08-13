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

# Participating in an Auction

This leverages Uniswap's Flash Swaps.

1. borrow DAI from Uniswap (`swap(x, 0, liquidatorContractAddress, abi.encode(user address))`)
2. Buy ETH at a discount from Yield Liquidation
3. Send the required ETH to Uniswap s.t xy = k holds!

# Demo

1. Deploy contracts
2. Deploy a Uniswap, mint a 50 ETH CDP @ 150 spot and draw 7500 DAI
3. As a user, put up 1 ETH collateral @ 150 spot, draw 150 yDAI
4. Now price moves against us
5. It gets detected by the liquidator -> triggers it
6. The liquidation gets detected -> tries to buy it with a flash loan -> returns that it's too expensive
7. Advance some time -> gets bought

NB: Each time a flashloan is done, we're borrowing DAI from Uniswap and returning ETH, so we're pushing DAI's price up. If you want to run more examples, do `dai.mint(pair.address, xxx)` followed by `pair.sync()` to update the reserves & price.
