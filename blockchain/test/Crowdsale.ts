import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { ethers } from "hardhat";

const ETHEREUM_UNISWAP_V3_FACTORY_ADDRESS =
  "0x1F98431c8aD98523631AE4a59f267346ea31F984";
const ETHEREUM_USDC_ADDRESS = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
const ETHEREUM_GRT_ADDRESS = "0xc944E90C64B2c07662A292be6244BDf05Cda44a7";
const ETHEREUM_WETH_ADDRESS = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

describe("Crowdsale", function () {
  async function getTokenAt(address: string) {
    return await ethers.getContractAt("IERC20Metadata", address);
  }

  async function deploy() {
    // Contracts are deployed using the first signer/account by default
    const [owner, otherAccount] = await ethers.getSigners();

    const TokenTicket = await ethers.getContractFactory("TokenTicket");
    const tokenTicket = await TokenTicket.connect(owner).deploy();

    const Crowdsale = await ethers.getContractFactory("Crowdsale");
    const crowdsale = await Crowdsale.connect(owner).deploy(
      ETHEREUM_UNISWAP_V3_FACTORY_ADDRESS,
      ETHEREUM_USDC_ADDRESS,
      ETHEREUM_WETH_ADDRESS,
      [ETHEREUM_GRT_ADDRESS],
      tokenTicket.address,
      10,
      100,
    );

    return { crowdsale, tokenTicket, owner, otherAccount };
  }

  it("Basic Test", async () => {
    const { crowdsale, tokenTicket, owner, otherAccount } = await loadFixture(
      deploy
    );

    const WETHtoken = await getTokenAt(ETHEREUM_WETH_ADDRESS);
    const USDCtoken = await getTokenAt(ETHEREUM_USDC_ADDRESS);

    const price = (await crowdsale.estimateAmountOut(
      ETHEREUM_WETH_ADDRESS,
      ETHEREUM_USDC_ADDRESS,
      3000,
      10n**BigInt(await WETHtoken.decimals()),
      3600
    )).toNumber() / (10**(await USDCtoken.decimals()));

    console.log(price);
  });

  it("get native price test", async () => {
    const { crowdsale, tokenTicket, owner, otherAccount } = await loadFixture(deploy);
    const USDCtoken = await getTokenAt(ETHEREUM_USDC_ADDRESS);

    const price = (await crowdsale.getETHPrice()).toNumber() / (10**(await USDCtoken.decimals()));

    console.log(price);
  });

  it("get custom token price test", async () => {
    const { crowdsale, tokenTicket, owner, otherAccount } = await loadFixture(deploy);
    const USDCtoken = await getTokenAt(ETHEREUM_USDC_ADDRESS);

    const price = (await crowdsale.getERC20Price(ETHEREUM_GRT_ADDRESS)).toNumber() / (10**(await USDCtoken.decimals()));

    console.log(price);
  });
});
