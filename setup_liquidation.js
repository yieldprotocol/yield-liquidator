const Treasury = artifacts.require('Treasury')
const WETH = artifacts.require('WETH9')
const Controller = artifacts.require('Controller')
const YDai = artifacts.require('YDai')
const Vat = artifacts.require('Vat')
const Liquidations = artifacts.require('Liquidations')

const { BigNumber } = require('ethers')
const { parseEther } = require("ethers/lib/utils")

const bWETH = web3.utils.fromAscii("ETH-A");
const bSpot = web3.utils.fromAscii("spot");

const toRay = (value) => {
  let exponent = BigNumber.from(10).pow(BigNumber.from(17))
  return BigNumber.from((value) * 10 ** 10).mul(exponent)
}

const initialSpot = 150
const liquidationSpot = 0.8 * initialSpot
const amt = 0.1;

module.exports = async (callback) => {
    try { 
    console.log('Creating liquidation opoprtunity')

    const accounts = await web3.eth.getAccounts()
    const user = accounts[0];

    // play with 0.1 eth each time

    const controller = await Controller.deployed();
    const treasury = await Treasury.deployed();

    // // get the weth
    const weth = await WETH.deployed()
    const wethAmount = parseEther(amt.toString());
    await weth.deposit({ from: user, value: wethAmount.toString() });
    await weth.approve(treasury.address, wethAmount, { from: user })
    await controller.post(bWETH, user, user, wethAmount, { from: user})

    // open the position
    const ydai = await YDai.deployed();
    const maturity = await ydai.maturity();
    const maxAmount = parseEther('15') // we put up 0.1 eth at 150 spot -> can borrow max 15
    await controller.borrow(bWETH, maturity, user, user, maxAmount, { from: user });

    // bump the oracle against us to trigger liquidation
    const vat = await Vat.deployed();
    await vat.file(bWETH, bSpot, toRay(liquidationSpot))

    // wait for the liquidation software to do the magic
    // it should 
    // 1. detect new user by the Posted/Borrowed event (who)
    // 2. Check their liq price (should?)
    // 3. Call `liquidate` (go)
    console.log("Opportunity:", user, bWETH)
    console.log("Is collateralized?", await controller.isCollateralized(bWETH, user))
    console.log("Controller:", controller.address);
    console.log("Liquidations:", Liquidations.address);
        callback()
    } catch (e) {console.log(e)}
}

