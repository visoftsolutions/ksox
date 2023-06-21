import { time, loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { expect } from "chai";
import { ethers } from "hardhat";

const splitSig = (sig: string) => {
  // splits the signature to r, s, and v values.
  const pureSig = sig.replace("0x", "");

  const r = Buffer.from(pureSig.substring(0, 64), "hex");
  const s = Buffer.from(pureSig.substring(64, 128), "hex");
  const v = parseInt(pureSig.substring(128, 130), 16).toString();

  return {
    r,
    s,
    v,
  };
};

describe("Lock", function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deployTreasuryFixture() {
    // Contracts are deployed using the first signer/account by default
    const [owner, otherAccount] = await ethers.getSigners();

    const TreasuryFactory = await ethers.getContractFactory("Treasury");
    const Treasury = await TreasuryFactory.deploy();

    const TokenFactory = await ethers.getContractFactory("Token");
    const Token = await TokenFactory.deploy("Token", "TOK");

    const TokenPermitFactory = await ethers.getContractFactory("TokenPermit");
    const TokenPermit = await TokenPermitFactory.deploy("TokenPermit", "TOKP");

    return { Treasury, owner, otherAccount, Token, TokenPermit };
  }

  describe("Treasury balanceOf", function () {
    it("zero funds", async function () {
      const { Treasury, Token, owner, otherAccount } = await loadFixture(deployTreasuryFixture);
      const tokenAddress = await Token.getAddress();
      expect(await Treasury.balanceOf(tokenAddress, owner.address)).to.equal(0);
      expect(await Treasury.balanceOf(tokenAddress, otherAccount.address)).to.equal(0);
    });
  });

  describe("Token balanceOf", function () {
    it("owner has funds", async function () {
      const { Token, owner } = await loadFixture(deployTreasuryFixture);
      expect(await Token.balanceOf(owner.address)).to.equal(100000n * 10n ** (await Token.decimals()));
    });

    it("others dont have funds", async function () {
      const { Token, otherAccount } = await loadFixture(deployTreasuryFixture);
      expect(await Token.balanceOf(otherAccount.address)).to.equal(0n);
    });
  });

  describe("TokenPermit balanceOf", function () {
    it("owner has funds", async function () {
      const { TokenPermit, owner } = await loadFixture(deployTreasuryFixture);
      expect(await TokenPermit.balanceOf(owner.address)).to.equal(100000n * 10n ** (await TokenPermit.decimals()));
    });

    it("others dont have funds", async function () {
      const { TokenPermit, otherAccount } = await loadFixture(deployTreasuryFixture);
      expect(await TokenPermit.balanceOf(otherAccount.address)).to.equal(0n);
    });
  });

  describe("Treasury deposit", function () {
    it("check balance after deposit", async function () {
      const { Treasury, Token, owner, otherAccount } = await loadFixture(deployTreasuryFixture);
      const tokenAddress = await Token.getAddress();
      const treasuryAddress = await Treasury.getAddress();
      const value = 10n * 10n ** (await Token.decimals());

      await Token.connect(owner).approve(treasuryAddress, value);
      await Treasury.connect(owner).deposit(tokenAddress, value);

      expect(await Treasury.balanceOf(tokenAddress, owner.address)).to.equal(value);
      expect(await Treasury.balanceOf(tokenAddress, otherAccount.address)).to.equal(0);
    });
  });

  describe("Treasury depositPermit", function () {
    it("check balance after depositPermit", async function () {
      const ONE_YEAR_IN_SECS = 365 * 24 * 60 * 60;
      const { Treasury, TokenPermit, owner, otherAccount } = await loadFixture(deployTreasuryFixture);
      const tokenAddress = await TokenPermit.getAddress();
      const treasuryAddress = await Treasury.getAddress();
      const value = 10n * 10n ** (await TokenPermit.decimals());
      const deadline = (await time.latest()) + ONE_YEAR_IN_SECS;
      const nonce = await TokenPermit.nonces(owner);

      const domain = {
        name: "TokenPermit",
        version: "1",
        chainId: 31337,
        verifyingContract: tokenAddress,
      };

      const permitType = [
        { name: "owner", type: "address" },
        { name: "spender", type: "address" },
        { name: "value", type: "uint256" },
        { name: "nonce", type: "uint256" },
        { name: "deadline", type: "uint256" },
      ];

      const permit = {
        owner: owner.address,
        spender: treasuryAddress,
        value: value,
        nonce: nonce,
        deadline: deadline,
      };

      const { r, s, v } = splitSig(await owner.signTypedData(domain, { Permit: permitType }, permit));
      await Treasury.depositPermit(tokenAddress, value, deadline, v, r, s);

      expect(await Treasury.balanceOf(tokenAddress, owner.address)).to.equal(value);
      expect(await Treasury.balanceOf(tokenAddress, otherAccount.address)).to.equal(0);
    });
  });

  describe("Treasury withdraw", function () {
    it("deposit withdraw duality", async function () {
      const { Treasury, Token, owner, otherAccount } = await loadFixture(deployTreasuryFixture);
      const tokenAddress = await Token.getAddress();
      const treasuryAddress = await Treasury.getAddress();
      const value = 10n * 10n ** (await Token.decimals());

      await Token.connect(owner).approve(treasuryAddress, value);
      await Treasury.connect(owner).deposit(tokenAddress, value);

      expect(await Treasury.balanceOf(tokenAddress, owner.address)).to.equal(value);
      expect(await Treasury.balanceOf(tokenAddress, otherAccount.address)).to.equal(0);
      expect(await Token.balanceOf(owner.address)).to.equal(100000n * 10n ** (await Token.decimals()) - value);

      await Treasury.withdraw(tokenAddress, owner.address, value);

      expect(await Treasury.balanceOf(tokenAddress, owner.address)).to.equal(0);
      expect(await Token.balanceOf(owner.address)).to.equal(100000n * 10n ** (await Token.decimals()));
    });
  });
});
