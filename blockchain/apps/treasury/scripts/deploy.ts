import { ethers } from "hardhat";

async function main() {
  const [owner] = await ethers.getSigners();

  const TokenFactory = await ethers.getContractFactory("Token");

  const bitcoinToken = await TokenFactory.deploy("Wrapped Bitcoin", "BTC");
  await bitcoinToken.waitForDeployment();
  await (await bitcoinToken.mint(owner, 1_000_000n * 10n ** 18n)).wait();
  console.log(
    "bitcoinToken Contract Address: ",
    await bitcoinToken.getAddress()
  );

  const etherToken = await TokenFactory.deploy("Wrapped Ether", "ETH");
  await etherToken.waitForDeployment();
  await (await etherToken.mint(owner, 1_000_000n * 10n ** 18n)).wait();
  console.log("etherToken Contract Address: ", await etherToken.getAddress());

  const usdtToken = await TokenFactory.deploy("Tether USD", "USDT");
  await usdtToken.waitForDeployment();
  await (await usdtToken.mint(owner, 1_000_000n * 10n ** 18n)).wait();
  console.log("usdtToken Contract Address: ", await usdtToken.getAddress());

  const TreasuryFactory = await ethers.getContractFactory("Treasury");
  const treasury = await TreasuryFactory.deploy("Treasury", owner);
  await treasury.waitForDeployment();
  console.log("Treasury Contract Address: ", await treasury.getAddress());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
