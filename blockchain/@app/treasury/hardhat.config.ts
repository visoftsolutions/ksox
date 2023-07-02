import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import { config as dotenvConfig } from "dotenv";

const ALCHEMY_API_KEY = dotenvConfig().parsed?.ALCHEMY_API_KEY;

const config: HardhatUserConfig = {
  solidity: "0.8.18",
  networks: {
    hardhat: {
      forking: {
        url: `https://eth-mainnet.alchemyapi.io/v2/${ALCHEMY_API_KEY}`,
        blockNumber: 17228270,
      },
      chainId: 31337,
      mining: {
        interval: 1000,
      },
    },
  },
};

export default config;
