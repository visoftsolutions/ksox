import { ethers } from "hardhat";

const ETHEREUM_UNISWAP_V3_FACTORY_ADDRESS ="0x1F98431c8aD98523631AE4a59f267346ea31F984";
const ETHEREUM_USDC_ADDRESS = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
const ETHEREUM_GRT_ADDRESS = "0xc944E90C64B2c07662A292be6244BDf05Cda44a7";
const ETHEREUM_WETH_ADDRESS = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

async function main() {
  const [owner] = await ethers.getSigners();
  console.log("Contracts owner:", owner.address);

  const TokenTicket = await ethers.getContractFactory("TokenTicket");
  const tokenTicket = await TokenTicket.connect(owner).deploy("KSX Ticket", "KSXT");
  await tokenTicket.deployed();
  console.log(`TokenTicket deployed with address ${tokenTicket.address}`);

  const Phase = await ethers.getContractFactory("Phase");
  const phase = await Phase.connect(owner).deploy(
    "Phase 1",
    ETHEREUM_UNISWAP_V3_FACTORY_ADDRESS,
    ETHEREUM_USDC_ADDRESS,
    ETHEREUM_WETH_ADDRESS,
    tokenTicket.address,
    10n, 10n,
    10n ** 18n,
    3, 2,
    [ETHEREUM_GRT_ADDRESS],
    10, 100
    );
  await phase.deployed();
  console.log(`Phase deployed with address ${phase.address}`);

  await tokenTicket.connect(owner).grantRole(await tokenTicket.MINTER_ROLE(), phase.address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
