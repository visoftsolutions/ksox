import { Chain, localhost } from "viem/chains";

export interface Network {
  network: Chain;
  icon: string;
}

export const AVAILABLE_CHAINS: Network[] = [
  {
    network: {
      id: 74207,
      name: "Testnet",
      network: "testnet",
      nativeCurrency: {
        decimals: 18,
        name: "Ether",
        symbol: "ETH",
      },
      rpcUrls: {
        default: {
          http: ["https://node.ksox.finance/"],
        },
        public: {
          http: ["https://node.ksox.finance/"],
        },
      },
    },
    icon: "/gfx/asset_icons/eth.svg",
  },
];
