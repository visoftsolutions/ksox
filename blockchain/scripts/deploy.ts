import { ethers } from "hardhat";

async function main() {
  const [owner] = await ethers.getSigners();
  console.log("Contracts owner:", owner.address);

  const TokenTicket = await ethers.getContractFactory("TokenTicket");
  const tokenticket = await TokenTicket.connect(owner).deploy("KSX Ticket", "KSXT");
  await tokenticket.deployed();
  console.log(`TokenTicket deployed with address ${tokenticket.address}`);

  const config = {
    _uniswapV3FactoryAddress: "0x1F98431c8aD98523631AE4a59f267346ea31F984",
    _referenceTokenAddress: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // uniswap usdc
    _wethAddress: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // uniswap weth
    _acceptedTokens: [
      "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599", // wrapped BTC on uniswap
      "0xe8A3Bf796cA5a13283ec6B1c5b645B91D7CfEf5D", // ZVT token on uniswap
    ],
    _soldTokenAddress: tokenticket.address,
  }
  const Phase = await ethers.getContractFactory("Phase");
  const phase = await Phase.connect(owner).deploy(config._uniswapV3FactoryAddress, config._referenceTokenAddress, config._wethAddress, config._acceptedTokens, config._soldTokenAddress);
  await phase.deployed();
  console.log(`Phase deployed with address ${phase.address}`);

  await tokenticket.connect(owner).grantRole(await tokenticket.MINTER_ROLE(), phase.address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
