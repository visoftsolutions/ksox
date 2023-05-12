import { loadFixture, time } from "@nomicfoundation/hardhat-network-helpers";
import { ethers } from "hardhat";
import { expect } from "chai";

const ETHEREUM_UNISWAP_V3_FACTORY_ADDRESS =
  "0x1F98431c8aD98523631AE4a59f267346ea31F984";
const ETHEREUM_USDC_ADDRESS = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
const ETHEREUM_GRT_ADDRESS = "0xc944E90C64B2c07662A292be6244BDf05Cda44a7";
const ETHEREUM_WETH_ADDRESS = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

describe("Phase", function () {
  async function getTokenAt(address: string) {
    return await ethers.getContractAt(
      "contracts/IERC20Metadata.sol:IERC20Metadata",
      address
    );
  }
  
  async function getWethAt(address: string) {
    return await ethers.getContractAt("contracts/IWETH.sol:IWETH", address);
  }

  async function getAccountBalance(address: string) {
    return await ethers.provider.getBalance(address);
  }

  async function deploy() {
    const [owner, otherAccount] = await ethers.getSigners();
    const TokenTicket = await ethers.getContractFactory("TokenTicket");
    const tokenTicket = await TokenTicket.connect(owner).deploy(
      "KSOX Ticket Token",
      "KSXT"
    );

    const Phase = await ethers.getContractFactory("Phase");
    const phase = await Phase.connect(owner).deploy(
      ETHEREUM_UNISWAP_V3_FACTORY_ADDRESS,
      ETHEREUM_USDC_ADDRESS,
      ETHEREUM_WETH_ADDRESS,
      [ETHEREUM_GRT_ADDRESS],
      tokenTicket.address,
      10,
      100,
      3,
      2,
      10n ** 6n * 10n ** 18n,
      12 * 3600
    );

    await tokenTicket
      .connect(owner)
      .grantRole(await tokenTicket.MINTER_ROLE(), phase.address);

    return { phase, tokenTicket, owner, otherAccount };
  }

  async function deployAndStart() {
    const [owner, otherAccount] = await ethers.getSigners();
    const TokenTicket = await ethers.getContractFactory("TokenTicket");
    const tokenTicket = await TokenTicket.connect(owner).deploy(
      "KSOX Ticket Token",
      "KSXT"
    );

    const Phase = await ethers.getContractFactory("Phase");
    const phase = await Phase.connect(owner).deploy(
      ETHEREUM_UNISWAP_V3_FACTORY_ADDRESS,
      ETHEREUM_USDC_ADDRESS,
      ETHEREUM_WETH_ADDRESS,
      [ETHEREUM_GRT_ADDRESS],
      tokenTicket.address,
      10,
      100,
      3,
      2,
      10n ** 6n * 10n ** 18n,
      12 * 3600
    );

    await tokenTicket
      .connect(owner)
      .grantRole(await tokenTicket.MINTER_ROLE(), phase.address);

    let startTimestamp = Math.round(Date.now() / 1000) + 13 * 3600;
    let finishTimestamp = startTimestamp + 12 * 3600;
    await phase
      .connect(owner)
      .startNewBucket(
        startTimestamp,
        finishTimestamp,
        10n ** 6n * 10n ** 18n,
        10,
        100
      );
    await time.setNextBlockTimestamp(startTimestamp);

    return { phase, tokenTicket, owner, otherAccount };
  }

  it("Conclude Current Bucket Throws When Not Active", async () => {
    const { phase } = await loadFixture(deploy);
    await expect(phase.concludeCurrentBucket()).to.be.revertedWith(
      "BUCKET_NOT_ACTIVE"
    );
  });

  it("Conclude Whole Phase Emits Event", async () => {
    const { phase } = await loadFixture(deploy);
    await expect(phase.concludeWholePhase())
      .to.emit(phase, "PhaseConcluded")
      .withArgs(0);
  });

  it("Withdraw Throws When Bucket Active", async () => {
    const { phase, otherAccount } = await loadFixture(deploy);
    const wethToken = await getTokenAt(ETHEREUM_WETH_ADDRESS);
    let startTimestamp = Math.round(Date.now() / 1000) + 13 * 3600;
    let finishTimestamp = startTimestamp + 12 * 3600;
    await phase.startNewBucket(
        startTimestamp,
        finishTimestamp,
        10n ** 6n * 10n ** 18n,
        10,
        100
      );
    await expect(
      phase.withdraw(wethToken.address, otherAccount.address, 100)
    ).to.be.revertedWith("BUCKET_ACTIVE");
  });

  it("Start New Bucket Emits Event", async () => {
    const { phase } = await loadFixture(deploy);
    let startTimestamp = Math.round(Date.now() / 1000) + 12 * 3600;
    let finishTimestamp = startTimestamp + 12 * 3600;
    await expect(
      phase.startNewBucket(
        startTimestamp,
        finishTimestamp,
        10n * 10n ** 18n,
        10,
        100
      )
    )
      .to.emit(phase, "NewBucketCreated")
      .withArgs(1, startTimestamp, finishTimestamp, 10n * 10n ** 18n, 10, 100);
  });

  it("Conclde current bucket", async () => {
    const { phase } = await loadFixture(deployAndStart);
    await expect(phase.concludeCurrentBucket())
      .to.emit(phase, "BucketConcluded")
      .withArgs(1, 0, 0);
  });

  it("But With ETH", async () => {
    const { phase, tokenTicket, owner, otherAccount } = await loadFixture(
      deploy
    );
    const wethToken = await getWethAt(ETHEREUM_WETH_ADDRESS);

    let startTimestamp = Math.round(Date.now() / 1000) + 13 * 3600;
    let finishTimestamp = startTimestamp + 12 * 3600;
    await phase
      .connect(owner)
      .startNewBucket(
        startTimestamp,
        finishTimestamp,
        10n ** 6n * 10n ** 18n,
        10,
        100
      );
    await time.setNextBlockTimestamp(startTimestamp);

    await wethToken.connect(otherAccount).approve(phase.address, 10n ** 18n);
    await phase.connect(otherAccount).buyWithETH({ value: 10n ** 18n });
    expect(await phase.isBucketActive()).to.be.true;
    expect(
      await tokenTicket.connect(otherAccount).balanceOf(otherAccount.address)
    ).to.be.equal(18418517370000000000000n);
  });

  it("But With ETH Whole Bucket", async () => {
    const { phase, tokenTicket, owner, otherAccount } = await loadFixture(
      deploy
    );
    const wethToken = await getWethAt(ETHEREUM_WETH_ADDRESS);

    let startTimestamp = Math.round(Date.now() / 1000) + 13 * 3600;
    let finishTimestamp = startTimestamp + 12 * 3600;
    let amountToSell = 10n ** 6n * 10n ** 18n;
    await phase
      .connect(owner)
      .startNewBucket(
        startTimestamp,
        finishTimestamp,
        amountToSell,
        10,
        100
      );
    await time.setNextBlockTimestamp(startTimestamp);
    expect(await phase.isBucketActive()).to.be.true;

    await wethToken.connect(otherAccount).approve(phase.address, 100n * 10n ** 18n);
    await phase.connect(otherAccount).buyWithETH({ value: 100n * 10n ** 18n });
    expect(await phase.isBucketActive()).to.be.false;
    expect(
      await tokenTicket.connect(otherAccount).balanceOf(otherAccount.address)
    ).to.be.equal(amountToSell);
  });
});
