import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.18",
  networks: {
    local: {
      url: "http://localhost:8545/",
    },
    testnet: {
      url: "https://node.ksox.finance/",
    },
    polygon: {
      url: "https://polygon-rpc.com/",
    }
  },
};

export default config;
