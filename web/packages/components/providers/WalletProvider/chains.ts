import { Chain, localhost } from "viem/chains";

export interface Network {
  network: Chain;
  icon: string;
}

export const AVAILABLE_CHAINS: Network[] = [
  {
    network: {
      id: 74207,
      name: "Localhost",
      network: "localhost",
      nativeCurrency: {
        decimals: 18,
        name: "Ether",
        symbol: "ETH",
      },
      rpcUrls: {
        default: {
          http: ["http://127.0.0.1:8545"],
        },
        public: {
          http: ["http://127.0.0.1:8545"],
        },
      },
    },
    icon: "/gfx/asset_icons/eth.svg",
  },
];
