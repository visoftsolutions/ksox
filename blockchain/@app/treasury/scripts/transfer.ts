import { ethers } from "hardhat";

const TOKEN_ADDRESS = "0xd8Cf62f758CFce316577f2769abbc23C3930CbdD";
const TREASURY_ADDRESS = "0x78cC73D4dD9D4fdb32158B7F7761F6CCf05e362C";
const AMOUNT = 1n * 10n ** 18n;

const splitSig = (sig: string) => {
  const pureSig = sig.replace("0x", "");
  const r = "0x" + pureSig.substring(0, 64);
  const s = "0x" + pureSig.substring(64, 128);
  const v = parseInt(pureSig.substring(128, 130), 16);
  return {
    r,
    s,
    v,
  };
};

async function main() {
  const [owner] = await ethers.getSigners();

  const Treasury = await ethers.getContractAt("Treasury", TREASURY_ADDRESS);
  console.log("Treasury: ", await Treasury.getAddress());

  const nonce = await Treasury.nonces(owner);

  const deadline = ((await ethers.provider.getBlock("latest"))?.timestamp ?? 0) + 3600;

  console.log(nonce, deadline);
  console.log(owner.address);

  const domain = {
    name: "Treasury",
    version: "1",
    chainId: 11155111,
    verifyingContract: TREASURY_ADDRESS,
  };

  const permitType = [
    { name: "owner", type: "address" },
    { name: "spender", type: "address" },
    { name: "value", type: "uint256" },
    { name: "nonce", type: "uint256" },
    { name: "deadline", type: "uint256" },
  ];

  const value = {
    owner: owner.address,
    spender: TREASURY_ADDRESS,
    value: AMOUNT,
    nonce,
    deadline,
  };

  const signature = await owner.signTypedData(domain, { Permit: permitType }, value);
  console.log(signature);

  const { r, s, v } = splitSig(signature);
  console.log(r, s, v);

  const transfer = await Treasury.connect(owner).depositPermit(TOKEN_ADDRESS, AMOUNT, deadline, v, r, s);
  console.log("Treasury Deposit txhash: ", transfer.hash);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
