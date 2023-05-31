import { ethers } from "hardhat";
import { ChainId, Fetcher, Token } from '@uniswap/sdk'




async function main() {
  const [owner] = await ethers.getSigners();
  console.log("Contracts owner:", owner.address);

  const chainId = ChainId.MAINNET
  console.log(`The chainId is ${chainId}.`)
  const tokenAddress = '0x6B175474E89094C44Da98b954EedeAC495271d0F' // must be checksummed

  // note that you may want/need to handle this async code differently,
  // for example if top-level await is not an option
  const DAI: Token = await Fetcher.fetchTokenData(chainId, tokenAddress, owner)

  console.log(DAI)
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
