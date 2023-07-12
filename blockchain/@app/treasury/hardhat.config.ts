import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import { config as dotenvConfig } from "dotenv";

const ALCHEMY_API_KEY_MAINNET = dotenvConfig().parsed?.ALCHEMY_API_KEY_MAINNET ?? "";
const ALCHEMY_API_KEY_SEPOLIA = dotenvConfig().parsed?.ALCHEMY_API_KEY_SEPOLIA ?? "";
const PRIVATE_KEY_SEPOLIA = dotenvConfig().parsed?.PRIVATE_KEY_SEPOLIA ?? "";

const config: HardhatUserConfig = {
  solidity: "0.8.18",
  networks: {
    hardhat: {
      forking: {
        url: `https://eth-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY_MAINNET}`,
        blockNumber: 17228270,
      },
      chainId: 31337,
      mining: {
        interval: 1000,
      },
    },
    sepolia: {
      url: `https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY_SEPOLIA}`,
      accounts: [PRIVATE_KEY_SEPOLIA]
    }
  },
};

export default config;
