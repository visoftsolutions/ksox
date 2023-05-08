import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
import { ethers } from "hardhat";

const ETHEREUM_USDC_ADDRESS = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
const ETHEREUM_WETH_ADDRESS = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

describe("Test", function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deploy() {
    const [owner, otherAccount] = await ethers.getSigners();
    const Test = await ethers.getContractFactory("Test");
    const test = await Test.connect(owner).deploy();
    return { test, owner, otherAccount };
  }

  async function sendEther(signer: SignerWithAddress, to: string, value: string) {
    const tx = { to, value };
    const txResponse = await signer.sendTransaction(tx);
    return await txResponse.wait();
  }

  describe("Deployment", function () {
    it("Should set the params", async function () {
      console.log(await ethers.getSigners());
      const { test, owner, otherAccount } = await loadFixture(deploy);
      console.log(await ethers.provider.getBalance(test.address));
      console.log(await await test.connect(otherAccount).deposit(1n ** 10n**18n));
      console.log(await ethers.provider.getBalance(test.address));
    });
  });
});
