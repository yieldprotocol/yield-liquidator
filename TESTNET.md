## Testing

In this guide, you will:
1. Deploy the yield contracts
2. Run the liquidator 
3. Create a liquidation opportunity
4. See the liquidator trigger the liquidation 
5. After some time, see the liquidator participate in the auction

### Deploy the contracts

First we must clone the contracts and install the deps:

```
git clone https://github.com/yieldprotocol/eDai
cd eDai
git checkout liquidator-testing
yarn
```

In one terminal, run ganache: `./scripts/ganache.sh`

In another terminal, deploy the contracts: `npx truffle migrate --reset`

This deploys MakerDAO, Yield, UniswapV2, [Multicall](https://github.com/makerdao/multicall) and the Flashloan contract.

Run `npx truffle exec scripts/addresses.js`. This will create a JSON file called `addrs.json`, containing the addresses of all the deployed contracts.

### Run the liquidator

In a new terminal, navigate back to the `yield-liquidator` directory.

You first need to create your private key and fund it with some ETH. Here, we'll
use a pre-funded key from ganache.

```
echo "0x8b6e036da61e5ac4874a770d7258583cd1b373798e740deaff4015fea80294b0" > private_key
```

Next, run the liquidator with the `addrs.json` file:

```
RUST_LOG=yield_liquidator=trace cargo run -- -c ../eDai/addrs.json -p ./private_key
```

You should see logs appear like below:

```
Oct 05 12:43:02.679  INFO yield_liquidator: Starting Yield Liquidator.
Oct 05 12:43:02.683  INFO yield_liquidator: Profits will be sent to 0x66a1b7374960cc1be80adca1de1be24d56e0f3d0
Oct 05 12:43:02.684  INFO yield_liquidator: Node: http://localhost:8545
Oct 05 12:43:02.687  INFO yield_liquidator: Controller: 0x9a0b52cf69aab3ff9b4b04d3cbc6352413733400
Oct 05 12:43:02.687  INFO yield_liquidator: Liquidations: 0xd45cf5045759c2a6a0213338a612d9ea1733c6c2
Oct 05 12:43:02.687  INFO yield_liquidator: Uniswap: 0x341a003891e2ed8cf4910afdf23d3cd9dde39862
Oct 05 12:43:02.687  INFO yield_liquidator: Multicall: Some(0x6107495d0f32d25ec6def1122ddfa21f42b50762)
Oct 05 12:43:02.688  INFO yield_liquidator: FlashLiquidator 0x03f32fcf400d2829631846a1bbff5d6b46f7638c
Oct 05 12:43:02.688  INFO yield_liquidator: Persistent data will be stored at: "data.json"
Oct 05 12:43:03.487 DEBUG eloop{block=93}: yield_liquidator::liquidations: checking for undercollateralized positions...
```

### Create a liquidation opportunity

In a new terminal, navigate back to the `eDai` directory.

Create a liquidation opportunity by running `npx truffle exec scripts/create_liquidation_opportunity.js`. This will borrow the max DAI possible at the current price, and then it will set the price to a lower value (via a backdoor method) so that the position is liquidatable. It will also fund Uniswap with some WETH/DAI at a favorable rate, so that an arbitrage is possible later on. You should see the following logs:

```
Oct 05 12:48:42.100 DEBUG eloop{block=109}: yield_liquidator::liquidations: new auction user=0x73bb8c9da5b0b3f5b05701e91f3925c3f247567b vault=Auction { started: 1601891322, debt: 150000000000000000000, collateral: 1000000000000000000 }
Oct 05 12:48:42.198  INFO eloop{block=109}: yield_liquidator::liquidations: found undercollateralized user. triggering liquidation user=0x73bb8c9da5b0b3f5b05701e91f3925c3f247567b debt_dai=150000000000000000000 max_debt_dai=120000000000000000000
Oct 05 12:48:42.559 TRACE eloop{block=109}: yield_liquidator::liquidations: Submitted liquidation tx_hash=0xff649dca811b8d81ae70e018e1824ad6a7853e2be274485802f1aa480477c878 user=0x73bb8c9da5b0b3f5b05701e91f3925c3f247567b
Oct 05 12:48:43.337 DEBUG eloop{block=109}:buying{user=0x73bb8c9da5b0b3f5b05701e91f3925c3f247567b auction_start=1601891322 auction_end=1601894922 debt=150000000000000000000}: yield_liquidator::liquidations: Auction not yet profitable via Uniswap flash swaps. price=300000000000000000000000030000
Oct 05 12:48:43.357 TRACE eloop{block=111}: yield_liquidator::liquidations: confirmed tx_hash=0xff649dca811b8d81ae70e018e1824ad6a7853e2be274485802f1aa480477c878 gas_used=139838 user=0x73bb8c9da5b0b3f5b05701e91f3925c3f247567b status="success" tx_type=liquidation
```

The above logs show that the liquidator detects a new borrower, then notices that they are undercollateralized and triggers their liquidation. It then immediately tries to participate in the auction for that user, but skips that step since it cannot make enough profit. Then, at block 111, it notifies us that the transaction for trigerring the liquidation has been mined (if it were not mined, it would automatically bump its gas price, until it gets mined).

### Participate in the auction

As shown in the previous logs, the current price does not yet allow for a profitable arbitrage
using Uniswap. Yield auction prices drop over time. We will now advance time on Ganache to the end
of the auction (notice how the argument is the same as the `auction_end` in the log above), and observe our liquidator participate and successfully reap the profit:

```
curl -H "Content-Type: application/json" -X POST --data '{"id":1337,"jsonrpc":"2.0","method":"evm_mine","params":[1601894922]}' http://localhost:8545
```

And on the liquidator:

```
Oct 05 12:53:41.110 TRACE eloop{block=112}:buying{user=0x73bb8c9da5b0b3f5b05701e91f3925c3f247567b auction_start=1601891322 auction_end=1601894922 debt=150000000000000000000}: yield_liquidator::liquidations: Submitted buy order tx_hash=0x3abc538b3debb0cbbf8442dcc52f89de8e98e755a664e2ff42f3c636b8f5dcfe
Oct 05 12:53:42.132 TRACE eloop{block=113}: yield_liquidator::liquidations: confirmed tx_hash=0x3abc538b3debb0cbbf8442dcc52f89de8e98e755a664e2ff42f3c636b8f5dcfe gas_used=393848 user=0x73bb8c9da5b0b3f5b05701e91f3925c3f247567b status="success" tx_type=auction
```

Success! We have liquidated and extracted a profit using funds from Uniswap. In order to guarantee a profit net of gas costs, consider passing a higher value to the `--min-profit` CLI argument.
