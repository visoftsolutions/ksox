import { ethers } from "hardhat";

async function main() {
  const [owner] = await ethers.getSigners();

  const TokenFactory = await ethers.getContractFactory("Token");

  const tokens = [
    await TokenFactory.deploy("Wrapped Bitcoin", "BTC"),
    await TokenFactory.deploy("Wrapped Ether", "ETH"),
    await TokenFactory.deploy("Tether USD", "USDT"),
  ];

  const TreasuryFactory = await ethers.getContractFactory("Treasury");
  const treasury = await TreasuryFactory.deploy("Treasury", owner);

  console.log("waiting for token contracts to be deployed...");
  for (let i = 0; i < tokens.length; i += 1) {
    const token = tokens[i];
    await token.waitForDeployment();
  }
  console.log("token contracts deployed");

  console.log("waiting for treasury contract to be deployed...");
  await treasury.waitForDeployment();
  console.log("treasury contract deployed");

  const accounts = await ethers.getSigners();
  const value = 1_000_000_000n * 10n ** 18n;
  let transactionResponses = [];

  console.log("minting tokens...");
  for (let i = 0; i < tokens.length; i += 1) {
    const token = tokens[i];
    for (let j = 0; j < accounts.length; j += 1) {
      const account = accounts[j];
      const contractTransactionResponse = await token.mint(
        account.address,
        value
      );
      transactionResponses.push(contractTransactionResponse);
    }
  }
  for (let i = 0; i < transactionResponses.length; i += 1) {
    const transactionResponse = transactionResponses[i];
    await transactionResponse.wait();
  }
  console.log("tokens minted");

  for (let i = 0; i < tokens.length; i += 1) {
    const token = tokens[i];
    console.log(`${await token.name()}: ${await token.getAddress()}`);
  }
  console.log(`${await treasury.name()}: ${await treasury.getAddress()}`);
  console.log(`PERMIT_TYPEHASH: ${await treasury.getTypeHash()}`);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
