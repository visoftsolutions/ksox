import { ethers } from "hardhat";

async function main() {
  const [owner] = await ethers.getSigners();
  console.log("Contracts owner:", owner.address);

  const Token = await ethers.getContractFactory("contracts/Token.sol:Token");

  const usdcToken = await Token.deploy("USDC", "USDC");
  console.log(`USDC deployed to: ${usdcToken.address}`);
  await usdcToken.grantRole(await usdcToken.MINTER_ROLE(), owner.address);
  await usdcToken.mint(owner.address, ethers.utils.parseEther("1000000000"));

  const usdtToken = await Token.deploy("USDT", "USDT");
  console.log(`USDT deployed to: ${usdtToken.address}`);
  await usdtToken.grantRole(await usdtToken.MINTER_ROLE(), owner.address);
  await usdtToken.mint(owner.address, ethers.utils.parseEther("1000000000"));

  const wbtcToken = await Token.deploy("WBTC", "WBTC");
  console.log(`WBTC deployed to: ${wbtcToken.address}`);
  await wbtcToken.grantRole(await wbtcToken.MINTER_ROLE(), owner.address);
  await wbtcToken.mint(owner.address, ethers.utils.parseEther("100000"));

  const UniswapV2Factory = await ethers.getContractFactory("UniswapV2Factory");
  const uniswapV2Factory = await UniswapV2Factory.deploy(owner.address);
  console.log(`UniswapV2Factory deployed to: ${uniswapV2Factory.address}`);

  await uniswapV2Factory.createPair(usdtToken.address, usdcToken.address);
  const usdtUsdcPairAddress = await uniswapV2Factory.getPair(usdtToken.address, usdcToken.address);
  const usdtUsdcPair = await ethers.getContractAt("UniswapV2Pair", usdtUsdcPairAddress);
  console.log(`USDT-USDC pair reserves: ${await usdtUsdcPair.getReserves()}`);

  const WETH9 = await ethers.getContractFactory("contracts/WETH9.sol:WETH9");
  const wethToken = await WETH9.deploy();
  console.log(`WETH deployed to: ${wethToken.address}`);

  const UniswapV2Router = await ethers.getContractFactory("UniswapV2Router02");
  const uniswapV2Router = await UniswapV2Router.deploy(uniswapV2Factory.address, wethToken.address);
  console.log(`UniswapV2Router02 deployed to: ${uniswapV2Router.address}`);

  await usdcToken.approve(uniswapV2Router.address, ethers.constants.MaxUint256);
  await usdtToken.approve(uniswapV2Router.address, ethers.constants.MaxUint256);
  await wbtcToken.approve(uniswapV2Router.address, ethers.constants.MaxUint256);
  await wethToken.approve(uniswapV2Router.address, ethers.constants.MaxUint256);

  console.log(await uniswapV2Router.addLiquidity(
    usdcToken.address,
    usdtToken.address,
    ethers.utils.parseEther("10000000"),
    ethers.utils.parseEther("10000000"),
    0,
    0,
    owner.address,
    Math.floor(Date.now() / 1000) + 60 * 10
  ));
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
