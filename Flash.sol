pragma solidity >=0.6.10;

interface LiquidationLike {
    function buy(
        address from,
        address to,
        address liquidated,
        uint256 daiAmount
    ) external returns (uint256);
}

interface PairLike {
    function token0() external returns (address);
    function token1() external returns (address);
    function getReserves() external view returns (uint112 _reserve0, uint112 _reserve1, uint32 _blockTimestampLast);
}

interface ERC20Like {
    // ERC20 approve
    function approve(address spender, uint256 amount) external returns (bool);

    // ERC20 transfer
    function transfer(address recipient, uint256 amount) external returns (bool);

    // WETH withdraw
    function withdraw(uint256 amount) external;
}

contract Flash {
    address immutable rewards;

    LiquidationLike immutable liquidation;
    PairLike pair;
    ERC20Like dai;
    ERC20Like weth;

    constructor(address treasury, address _pair, address _liquidation) public {
        pair = PairLike(_pair);
        liquidation = LiquidationLike(_liquidation);
        rewards = msg.sender;
        dai = ERC20Like(pair.token0());
        weth = ERC20Like(pair.token1());

        // allow the treasury to pull funds
        dai.approve(treasury, type(uint).max);
    }

    // amount0 is always DAI for the DAI-WETH pair
    function uniswapV2Call(address sender, uint amount0, uint amount1, bytes calldata data) external {
        require(msg.sender == address(pair), "ONLY_PAIR");
        require(amount1 == 0, "NON_ZERO_WETH_RECEIVED");

        // get the person being liquidated
        (address user, uint minProfitETH) = abi.decode(data, (address, uint));

        // send the received DAI to the auction and receive WETH (at a discount)
        uint wethReceived = liquidation.buy(address(this), address(this), user, amount0);

        // -> Calculate the amount of WETH required
        //
        //          wethReserves * daiAmount * 1000
        // weth = -----------------------------------
        //          (daiReserves - daiAmount) * 997
        (uint112 daiReserves, uint112 wethReserves,) = pair.getReserves();
        uint numerator = wethReserves * amount0 * 1000;
        uint denominator = (daiReserves - amount0) * 997;
        uint wethAmount = numerator / denominator + 1;

        require(wethReceived > wethAmount + minProfitETH, "NOT_ENOUGH_PROFIT");

        // pay back the loan
        require(weth.transfer(address(pair), wethAmount), "WETH_PAYBACK_FAILED");

        // reap the profit!
        uint profit = wethReceived - wethAmount;
        weth.withdraw(profit);
        (bool success,) = sender.call{value: profit}(new bytes(0));
        require(success, "COULD NOT WITHDRAW");
    }

    receive() external payable {}
}
