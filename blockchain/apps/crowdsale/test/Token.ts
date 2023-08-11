import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

describe("Token", function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deployToken() {
    // Contracts are deployed using the first signer/account by default
    const [owner, otherAccount] = await ethers.getSigners();

    const Token = await ethers.getContractFactory("Token");
    const token = await Token.connect(owner).deploy();

    return { token, owner, otherAccount };
  }

  describe("Deployment", function () {
    it("Should set the right totalSupply", async function () {
      const { token } = await loadFixture(deployToken);

      expect(await token.totalSupply()).to.equal(100n * 10n ** 18n);
    });
    it("Should set the right balance", async function () {
      const { token, owner } = await loadFixture(deployToken);

      expect(await token.balanceOf(owner.address)).to.equal(100n * 10n ** 18n);
    });
  });
});
