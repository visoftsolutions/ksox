import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { ethers } from "hardhat";
import { expect } from "chai";

const ETHEREUM_UNISWAP_V3_FACTORY_ADDRESS =
  "0x1F98431c8aD98523631AE4a59f267346ea31F984";
const ETHEREUM_USDC_ADDRESS = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
const ETHEREUM_WETH_ADDRESS = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

describe("UniswapV3Twap", function () {
  async function deploy() {
    // Contracts are deployed using the first signer/account by default
    const [owner, otherAccount] = await ethers.getSigners();

    const SimpleOracle = await ethers.getContractFactory("SimpleOracle");
    const simpleOracle = await SimpleOracle.connect(owner).deploy(
      ETHEREUM_UNISWAP_V3_FACTORY_ADDRESS,
      ETHEREUM_USDC_ADDRESS,
      ETHEREUM_WETH_ADDRESS,
      3000
    );

    return { simpleOracle, owner, otherAccount };
  }

  it("Basic Test", async () => {
    const { simpleOracle } = await loadFixture(deploy);

    const price = await simpleOracle.estimateAmountOut(
      ETHEREUM_WETH_ADDRESS,
      10n ** 18n,
      3600
    );

    expect(price).to.equal(1844616450);
  });
});
