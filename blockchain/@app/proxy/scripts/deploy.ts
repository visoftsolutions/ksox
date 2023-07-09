import { ethers, upgrades } from "hardhat";

const ADDRESS = "0x04C89607413713Ec9775E14b954286519d836FEf"

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);

  const MyToken = await ethers.getContractFactory("MyToken");
  const myToken = await upgrades.deployProxy(MyToken, [deployer.address]);
  await myToken.waitForDeployment();
  console.log("MyToken deployed to:", await myToken.getAddress());

}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
