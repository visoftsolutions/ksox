import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: {
    compilers: [{ version: "0.8.18" }, { version: "0.7.6" }, { version: "0.6.6" }, { version: "0.5.16" }],
  },
};

export default config;
