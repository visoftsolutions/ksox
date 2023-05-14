import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import { config as dotenvConfig } from "dotenv";

const ALCHEMY_API_KEY = dotenvConfig().parsed?.ALCHEMY_API_KEY;

const config: HardhatUserConfig = {
  solidity: {
    compilers: [
      { version: "0.8.18" },
      { version: "0.7.6" },
      { version: "0.6.6" },
      { version: "0.5.16" },
    ],
  },
  networks: {
    hardhat: {
      forking: {
        url: `https://eth-mainnet.alchemyapi.io/v2/${ALCHEMY_API_KEY}`,
        blockNumber: 17228270,
      },
      chainId: 1337,
      accounts: [
        {
          privateKey:
            "0x5a4637f4d5260f030ac93a057ceadcd6fb4c8c2a8b4a5daee7e794e953a65bc9",
          balance: (1000000n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x442d8382347067a9e12b63dfc36da4640b4e5b392a5488e9149754c7d48446ca",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x8a1925b703ac81006eff77677aecf1e2ced46d8b215c18929e49f8d3601312dc",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0xa9a50f68e59ac7b37a81506c96e96b592cfff4dd914b6aea547c24513d3570b8",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x104868793c2e3bc42f0edc0a1b29562564916584f1ec901995bb3c189b504df7",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x40b75e61e11454acdf6d4d3be395cdd7525016adc0ba42e7c9e135aa3d141be9",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x9f3ba95a032f68b7edaa77740584e0ddb3e5c36b4ecfed3ff8f33d4d66e4936e",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0xefe77fbbd662661a1ef2a1ac2a5c1216855b55c23acc0f549772e8f00bae185a",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x69fc2992e1178bc24cccf2a63bf9b085abed52c4139695d80f2feb21d8d6fb3a",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0xc8222d3730b4a1d5959c8fac7737691a65c8c4c4f3bbc1c41c3a00c9f7e7a666",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0xa8a5a16f66d251035659b41e9d1226305e9bd1df892c98fd76b75fccde074785",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x4ea9e9ba485f61a73165f65d3872ae6e65809e432db1dbe4d39ab1c3383d4c72",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0xef7487f123117408c51208160b9659e75ced095a814ea42c240404b2c8bd5756",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x8f4b13247df7ff7e0cc205236c669354ec31a670854d47e80ab166ff2f63e9aa",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x2da68d450228e5e60e291d218d3fb2c513e744521914a492020fd13a0ee51917",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0xc430bf7f3fe630b62a31390d4a23081fc26c88292cf46932a16d8fc4fd87e408",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x94c31acf7072e073b65d359a0e2c3e56a0dccb2ac8283b6a3ad2dd6cc76131a3",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x4e712e65390149d73e5f97015d5a82d03e4238b853f85cb514524f0c92022753",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x3981a8f7baa07694062da804ce382e4346ef6170f8bd45752f6256c1ed00e623",
          balance: (100n * 10n ** 18n).toString(),
        },
        {
          privateKey:
            "0x8d1a01b586d315e120966117f053f0d3cdd7ba979fb23f5f86d926e52c092097",
          balance: (100n * 10n ** 18n).toString(),
        },
      ],
    },
  },
};

export default config;
