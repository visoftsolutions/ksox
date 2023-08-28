import { ethers } from "hardhat";

const CONTRACT_ADDRESS_LIST = [
  "0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270",
  "0x7ceb23fd6bc0add59e62ac25578270cff1b9f619",
  "0x1bfd67037b42cf73acf2047067bd4f2c47d9bfd6",
  "0x2791bca1f2de4661ed88a30c99a7a9449aa84174",
  "0xc2132d05d31c914a87c6611c10748aeb04b58e8f",
];

async function main() {
  for (let i = 0; i < CONTRACT_ADDRESS_LIST.length; i += 1) {
    const contractAddress = CONTRACT_ADDRESS_LIST[i];
    const contract = await ethers.getContractAt("ERC20", contractAddress);
    console.log(`address ${contractAddress}`);
    console.log(`name: ${await contract.name()}`);
    console.log(`symbol: ${await contract.symbol()}`);
    console.log(`decimals: ${await contract.decimals()}`);
    console.log("");
  }
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
