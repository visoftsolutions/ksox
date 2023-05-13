import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: {
    compilers: [{ version: "0.8.18" }, { version: "0.7.6" }, { version: "0.6.6" }, { version: "0.5.16" }],
  },
  networks: {
    hardhat: {
      chainId: 1337,
      accounts: [
        { privateKey: "23c0a7ebc9e989d0b9dbcf6a4fb9a76daf1049222ce1e7450fd5ee555045069b", balance: (1000000n * 10n ** 18n).toString() },
        { privateKey: "92a7ae46548ab210cbaaa69a708388c588a12d698fdfbf081783610f88e85e04", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "c433215a9deef8018e5a659477b395ee4ae5c766155fbc68160c29516547c93d", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "ba281993a9ac28423fdd79d5b3b8cba00758288b73ae82cec51be3e11f3180ff", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "df7acd7d861baf145fc6e59e8ae0d5f9ec78f3ff620cabe1bb37a72e45487718", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "5f52c9f04fe444e817d7209772b71eb70acab28f6204aa1e561de97c90bd040a", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "2be955a1d887e9ec9808858d43ee5c8e812c073eefcd8319d9bd629fcf635dbb", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "5c77d0a796389eac78a54a0ba0c06b91c6cd2cd7347ed4ce78007aac343157e6", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "e2f0d93adbd42e05f58cbe7daa2a14798684f6d079245ec15f8f891a93a71ea5", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "721a735426b114518c830c342b303d027cb72ccf2a2cd9f6ff15ee9ad2dff133", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "0247a324c8e698439b9052c2e3c39ec4db927456b4dd3b6139f1cc2fdbd5b71e", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "9381da1c14f64c4c9f3a7d9c82a2d2246e07ff194760fbb85b9834b2f2ed169b", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "feb7ebdfab3180b3563095700b3d46e611abd4041c8005ac496f9a5c47e7fb53", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "f9c7016bd9df24caf9214b55edd05b1e105078889d62a2db4005d11b62c758c8", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "49917319801829ecec13e7a90565e6c5dc18de35b21143fc7c83f2c084bd9ce0", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "12bcbf7e68a1d78451fa0a2530569a60e9982825f2ebebbd54b4b1562e6f17c3", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "cc5a36bfe8e0dc9a10343889a2baf9a92c9d7f300dfb516925655eefa965f60d", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "9b490711e1420489b9b891e5b7fd4defd731d987d1680ba3bcba00bd5699d124", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "470236b6efb936bf241374517a0da08fca9b5fc6b47fa1d4f11e0815241a3638", balance: (100n * 10n ** 18n).toString() },
        { privateKey: "1525f447ecf1f934d436e70e2521a4781f7aee2f8d43590ffa3e2118c6972637", balance: (100n * 10n ** 18n).toString() },
      ]
    },
    // testnet: {
    //   url: "https://test-blockchain.ksox.exchange/",
    //   chainId: 1337,
    //   accounts: [
    //     { privateKey: "23c0a7ebc9e989d0b9dbcf6a4fb9a76daf1049222ce1e7450fd5ee555045069b", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "92a7ae46548ab210cbaaa69a708388c588a12d698fdfbf081783610f88e85e04", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "c433215a9deef8018e5a659477b395ee4ae5c766155fbc68160c29516547c93d", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "ba281993a9ac28423fdd79d5b3b8cba00758288b73ae82cec51be3e11f3180ff", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "df7acd7d861baf145fc6e59e8ae0d5f9ec78f3ff620cabe1bb37a72e45487718", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "5f52c9f04fe444e817d7209772b71eb70acab28f6204aa1e561de97c90bd040a", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "2be955a1d887e9ec9808858d43ee5c8e812c073eefcd8319d9bd629fcf635dbb", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "5c77d0a796389eac78a54a0ba0c06b91c6cd2cd7347ed4ce78007aac343157e6", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "e2f0d93adbd42e05f58cbe7daa2a14798684f6d079245ec15f8f891a93a71ea5", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "721a735426b114518c830c342b303d027cb72ccf2a2cd9f6ff15ee9ad2dff133", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "0247a324c8e698439b9052c2e3c39ec4db927456b4dd3b6139f1cc2fdbd5b71e", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "9381da1c14f64c4c9f3a7d9c82a2d2246e07ff194760fbb85b9834b2f2ed169b", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "feb7ebdfab3180b3563095700b3d46e611abd4041c8005ac496f9a5c47e7fb53", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "f9c7016bd9df24caf9214b55edd05b1e105078889d62a2db4005d11b62c758c8", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "49917319801829ecec13e7a90565e6c5dc18de35b21143fc7c83f2c084bd9ce0", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "12bcbf7e68a1d78451fa0a2530569a60e9982825f2ebebbd54b4b1562e6f17c3", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "cc5a36bfe8e0dc9a10343889a2baf9a92c9d7f300dfb516925655eefa965f60d", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "9b490711e1420489b9b891e5b7fd4defd731d987d1680ba3bcba00bd5699d124", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "470236b6efb936bf241374517a0da08fca9b5fc6b47fa1d4f11e0815241a3638", balance: (100n * 10n ** 18n).toString() },
    //     { privateKey: "1525f447ecf1f934d436e70e2521a4781f7aee2f8d43590ffa3e2118c6972637", balance: (100n * 10n ** 18n).toString() },
    //   ]
    // },
  },
};

export default config;
