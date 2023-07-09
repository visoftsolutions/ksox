import { ethers, upgrades } from "hardhat";

const ADDRESS = "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);

  const MyToken = await ethers.getContractFactory("MyToken");
  const myToken = await upgrades.upgradeProxy(ADDRESS, MyToken);
  await myToken.waitForDeployment();
  console.log("MyToken deployed to:", await myToken.getAddress());

}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
