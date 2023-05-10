import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

describe("TokenTicket", function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deployToken() {
    // Contracts are deployed using the first signer/account by default
    const [owner, otherAccount] = await ethers.getSigners();

    const TokenTicket = await ethers.getContractFactory("TokenTicket");
    const tokenTicket = await TokenTicket.connect(owner).deploy("KSOX Ticket Token", "KSXT");

    return { tokenTicket, owner, otherAccount };
  }

  describe("Deployment", function () {
    it("Should set the right totalSupply", async function () {
      const { tokenTicket } = await loadFixture(deployToken);

      expect(await tokenTicket.totalSupply()).to.equal(0);
    });
    it("Should set the right balance", async function () {
      const { tokenTicket, owner } = await loadFixture(deployToken);

      expect(await tokenTicket.balanceOf(owner.address)).to.equal(0);
    });
  });
});
