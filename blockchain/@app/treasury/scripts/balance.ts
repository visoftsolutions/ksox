import { ethers } from "hardhat";

const TOKEN_ADDRESS = "0x9E3E576d24A052fE6a0c88283D51a998FD69312b";
const TREASURY_ADDRESS = "0x8C7419dA64d29BE79a4a7933cC7cd92Fc31972A0";
const AMOUNT = 1n*10n**18n;

async function main() {
  const [owner] = await ethers.getSigners();

  const Treasury = await ethers.getContractAt("Treasury", TREASURY_ADDRESS);
  console.log("Treasury: ", await Treasury.getAddress());
  
  const Token = await ethers.getContractAt("Token", TOKEN_ADDRESS);
  console.log("Token: ", await Token.getAddress());

  let treasury_balance = await Treasury.balanceOf(TOKEN_ADDRESS, owner.address)
  console.log(`Treasury balance: ${TOKEN_ADDRESS}  ${owner.address}  ${treasury_balance}`);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
