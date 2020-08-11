pragma solidity >=0.6.10;

interface LiquidationLike {
    function buy(address from, address to, address liquidated, uint256 daiAmount) external;
}

interface PairLike {
    function token0() external returns (address);
    function token1() external returns (address);
    function getReserves() external view returns (uint112 _reserve0, uint112 _reserve1, uint32 _blockTimestampLast);
}

interface ERC20Like {
    function transfer(address recipient, uint256 amount) external returns (bool);
}

contract Flash {
    PairLike immutable pair;
    address immutable rewards;
    LiquidationLike immutable liquidation;

    ERC20Like immutable dai;
    ERC20Like immutable weth;

    constructor(address _pair, address _liquidation) public {
        pair = PairLike(_pair);
        liquidation = LiquidationLike(_liquidation);
        rewards = msg.sender;
        dai = ERC20Like(pair.token0());
        weth = ERC20Like(pair.token1());
    }

    // amount0 is always DAI for the DAI-WETH pair
    function uniswapV2Call(address sender, uint amount0, uint amount1, bytes calldata data) external {
        require(msg.sender == address(pair), "ONLY_PAIR");
        require(amount1 == 0, "NON_ZERO_WETH_RECEIVED");

        // get the person being liquidated
        address user = abi.decode(data, (address));
        
        // send the received DAI to the auction and receive WETH (at a discount)
        uint wethReceived = liquidation.buy(address(this), rewards, user, amount0);

        // Calculate the amount of WETH required:
        // (same logic as https://github.com/Uniswap/uniswap-v2-periphery/blob/master/contracts/libraries/UniswapV2Library.sol#L55-L59)
        (uint112 daiReserves, uint112 wethReserves,) = pair.getReserves();
        uint numerator = daiReserves * amount0 * 1000;
        uint denominator = (wethReserves - amount0) * 997;
        uint wethAmount = numerator / denominator + 1;
        require(wethReceived > wethAmount, "NOT_ENOUGH_PROFIT");

        // pay back the loan
        require(weth.transfer(address(pair), wethAmount), "WETH_PAYBACK_FAILED");

        // reap the profit!
        (bool success,) = sender.call{value: amountReceived - amountRequired}(new bytes(0));
        assert(success);
    }
}
