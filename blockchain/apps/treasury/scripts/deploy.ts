import { ethers } from "hardhat";

async function main() {
  const [owner] = await ethers.getSigners();

  const TreasuryFactory = await ethers.getContractFactory("Treasury");
  const treasury = await TreasuryFactory.deploy("Treasury", owner);
  await treasury.waitForDeployment();
  console.log("Treasury Contract Address: ", await treasury.getAddress());

  const TokenPermitFactory = await ethers.getContractFactory("TokenPermit");
  const tokenPermit = await TokenPermitFactory.deploy("TokenPermit", "TOKP");
  await tokenPermit.waitForDeployment();
  console.log("tokenPermit Contract Address: ", await tokenPermit.getAddress());

  await (await tokenPermit.mint(owner.address, 100000n * 10n ** 18n)).wait();
  console.log(`tokenPermit: ${owner.address} balance: ${await tokenPermit.balanceOf(owner.address)}`);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
