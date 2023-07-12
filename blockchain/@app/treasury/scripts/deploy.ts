import { ethers } from "hardhat";

async function main() {
  const [owner] = await ethers.getSigners();

  const TreasuryFactory = await ethers.getContractFactory("Treasury");
  const Treasury = await TreasuryFactory.deploy();
  await Treasury.waitForDeployment();
  console.log("Treasury: ", await Treasury.getAddress());

  const TokenFactory = await ethers.getContractFactory("Token");
  const Token = await TokenFactory.deploy("Token", "TOK");
  await Token.waitForDeployment();
  console.log("Token: ", await Token.getAddress());

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
