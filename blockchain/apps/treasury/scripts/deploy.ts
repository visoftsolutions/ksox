import { ethers } from "hardhat";

async function main() {
  const [owner] = await ethers.getSigners();

  const TokenFactory = await ethers.getContractFactory("Token");

  const bitcoinToken = await TokenFactory.deploy("Wrapped Bitcoin", "BTC");
  await bitcoinToken.waitForDeployment();
  console.log(
    "bitcoinToken Contract Address: ",
    await bitcoinToken.getAddress()
  );

  const etherToken = await TokenFactory.deploy("Wrapped Ether", "ETH");
  await etherToken.waitForDeployment();
  console.log("etherToken Contract Address: ", await etherToken.getAddress());

  const usdtToken = await TokenFactory.deploy("Tether USD", "USDT");
  await usdtToken.waitForDeployment();
  console.log("usdtToken Contract Address: ", await usdtToken.getAddress());

  const TreasuryFactory = await ethers.getContractFactory("Treasury");
  const treasury = await TreasuryFactory.deploy("Treasury", owner);
  await treasury.waitForDeployment();
  console.log("Treasury Contract Address: ", await treasury.getAddress());

  const tokens = [bitcoinToken, etherToken, usdtToken];
  const accounts = await ethers.getSigners();
  const value = 1_000_000_000n * 10n ** 18n;
  let transactionResponses = [];
  for (let i = 0; i < tokens.length; i += 1) {
    const tokenName = await tokens[i].name();
    for (let j = 0; j < accounts.length; j += 1) {
      const account = accounts[i];
      transactionResponses.push(
        await usdtToken.mint(account.address, 1_000_000_000n * 10n ** 18n)
      );
      console.log(`Minting ${value} of ${tokenName} to ${account.address}`);
    }
  }
  console.log("Finishing...");
  for (let i = 0; i < transactionResponses.length; i += 1) {
    await transactionResponses[i].wait();
  }
  console.log("Finished");
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
