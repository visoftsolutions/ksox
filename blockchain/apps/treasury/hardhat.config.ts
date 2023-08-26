import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.18",
  networks: {
    local: {
      url: "http://localhost/",
    },
    testnet: {
      url: "https://node.ksox.finance/",
    }
  },
};

export default config;
