import {
  time,
  loadFixture,
} from "@nomicfoundation/hardhat-toolbox/network-helpers";
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

describe("Permit Deposit & Permit Withdraw Mechanisms", function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deployTreasuryFixture() {
    const { chainId } = await ethers.provider.getNetwork();
    const [owner, user, otherUser, signingKey] = await ethers.getSigners();

    const WethFactory = await ethers.getContractFactory("WETH");
    const weth = await WethFactory.connect(owner).deploy("WETH", "WETH");
    await weth.waitForDeployment();

    const TokenFactory = await ethers.getContractFactory("Token");
    const token = await TokenFactory.connect(owner).deploy("MyToken", "MTK");
    await token.waitForDeployment();

    const TreasuryFactory = await ethers.getContractFactory("Treasury");
    const treasury = await TreasuryFactory.connect(owner).deploy(
      "Treasury",
      await weth.getAddress(),
      signingKey.address
    );
    await treasury.waitForDeployment();

    return {
      chainId,
      owner,
      user,
      otherUser,
      signingKey,
      weth,
      token,
      treasury,
    };
  }

  async function deployTreasuryFixturePremint() {
    const { chainId } = await ethers.provider.getNetwork();
    const [owner, user, otherUser, signingKey] = await ethers.getSigners();

    const WethFactory = await ethers.getContractFactory("WETH");
    const weth = await WethFactory.connect(owner).deploy("WETH", "WETH");
    await weth.waitForDeployment();

    const TokenFactory = await ethers.getContractFactory("Token");
    const token = await TokenFactory.connect(owner).deploy("MyToken", "MTK");
    await token.waitForDeployment();

    const TreasuryFactory = await ethers.getContractFactory("Treasury");
    const treasury = await TreasuryFactory.connect(owner).deploy(
      "Treasury",
      await weth.getAddress(),
      signingKey.address
    );
    await treasury.waitForDeployment();

    const MINT_AMOUNT = 100n * 10n ** (await token.decimals());
    await token.connect(owner).mint(user, MINT_AMOUNT);

    return {
      chainId,
      owner,
      user,
      otherUser,
      signingKey,
      weth,
      token,
      treasury,
    };
  }

  describe("Token", () => {
    it("zero funds", async function () {
      const { owner, user, token } = await loadFixture(deployTreasuryFixture);
      expect(await token.balanceOf(owner.address)).to.equal(0);
      expect(await token.balanceOf(user.address)).to.equal(0);
    });
    it("mint funds", async function () {
      const { owner, user, token } = await loadFixture(deployTreasuryFixture);
      await token.connect(owner).mint(user, 1n * 10n ** 18n);
      expect(await token.balanceOf(owner.address)).to.equal(0);
      expect(await token.balanceOf(user.address)).to.equal(1n * 10n ** 18n);
    });
  });

  describe("Treasury", () => {
    it("permit deposit funds", async function () {
      const { chainId, user, token, treasury } = await loadFixture(
        deployTreasuryFixturePremint
      );

      const ONE_YEAR_IN_SECS = 365 * 24 * 60 * 60;
      const deadline = (await time.latest()) + ONE_YEAR_IN_SECS;
      const nonce = await token.connect(user).nonces(user);
      const tokenAddress = await token.getAddress();
      const tokenName = await token.name();
      const treasuryAddress = await treasury.getAddress();
      const value = 10n * 10n ** (await token.connect(user).decimals());

      const domain = {
        name: tokenName,
        version: "1",
        chainId: chainId,
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
        owner: user.address,
        spender: treasuryAddress,
        value: value,
        nonce: nonce,
        deadline: deadline,
      };

      const { r, s, v } = splitSig(
        await user.signTypedData(domain, { Permit: permitType }, permit)
      );
      await treasury
        .connect(user)
        .depositPermit(tokenAddress, value, deadline, v, r, s);
      expect(await token.balanceOf(treasury)).to.equal(value);
    });

    it("permit withdraw funds", async function () {
      const { chainId, user, otherUser, signingKey, token, treasury } =
        await loadFixture(deployTreasuryFixturePremint);

      const value = 10n * 10n ** (await token.connect(user).decimals());
      {
        const ONE_YEAR_IN_SECS = 365 * 24 * 60 * 60;
        const deadline = (await time.latest()) + ONE_YEAR_IN_SECS;
        const nonce = await token.nonces(user);
        const tokenAddress = await token.getAddress();
        const tokenName = await token.name();
        const treasuryAddress = await treasury.getAddress();

        const domain = {
          name: tokenName,
          version: "1",
          chainId: chainId,
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
          owner: user.address,
          spender: treasuryAddress,
          value: value,
          nonce: nonce,
          deadline: deadline,
        };

        const { r, s, v } = splitSig(
          await user.signTypedData(domain, { Permit: permitType }, permit)
        );
        await treasury
          .connect(user)
          .depositPermit(tokenAddress, value, deadline, v, r, s);
      }
      expect(await token.balanceOf(treasury)).to.equal(value);
      {
        const ONE_YEAR_IN_SECS = 365 * 24 * 60 * 60;
        const deadline = (await time.latest()) + ONE_YEAR_IN_SECS;
        const nonce = await treasury.nonces(user.address);
        const tokenAddress = await token.getAddress();
        const treasuryAddress = await treasury.getAddress();
        const treasuryName = await treasury.name();

        const domain = {
          name: treasuryName,
          version: "1",
          chainId: chainId,
          verifyingContract: treasuryAddress,
        };

        const permitType = [
          { name: "owner", type: "address" },
          { name: "spender", type: "address" },
          { name: "token", type: "address" },
          { name: "value", type: "uint256" },
          { name: "nonce", type: "uint256" },
          { name: "deadline", type: "uint256" },
        ];

        const permit = {
          owner: signingKey.address,
          spender: user.address,
          token: tokenAddress,
          value: value,
          nonce: nonce,
          deadline: deadline,
        };

        const { r, s, v } = splitSig(
          await signingKey.signTypedData(domain, { Permit: permitType }, permit)
        );
        await treasury
          .connect(user)
          .withdrawPermit(tokenAddress, value, deadline, v, r, s, otherUser);
      }
      expect(await token.balanceOf(treasury)).to.equal(0);
      expect(await token.balanceOf(otherUser)).to.equal(value);
    });
  });
});
