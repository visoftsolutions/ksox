import { ethers } from "hardhat";

async function main() {
  const [owner] = await ethers.getSigners();

  const TreasuryFactory = await ethers.getContractFactory("Treasury");
  const Treasury = await TreasuryFactory.deploy("Treasury", "0x3acadfb15e991e8403d2fe3e75ee4782b88cf5b1");
  await Treasury.waitForDeployment();
  console.log("Treasury: ", await Treasury.getAddress());

  const TokenPermitFactory = await ethers.getContractFactory("TokenPermit");
  const TokenPermit = await TokenPermitFactory.deploy("TokenPermit", "TOKP");
  await TokenPermit.waitForDeployment();
  console.log("TokenPermit: ", await TokenPermit.getAddress());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
