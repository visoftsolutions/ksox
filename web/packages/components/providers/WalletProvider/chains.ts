import { Chain, mainnet, polygon, bsc, sepolia } from "viem/chains";

export interface Network {
  network: Chain;
  icon: string;
}

export const AVAILABLE_CHAINS: Network[] = [
  {
    network: mainnet,
    icon: "/gfx/asset_icons/eth.svg",
  },
  {
    network: polygon,
    icon: "/gfx/asset_icons/matic.svg",
  },
  {
    network: bsc,
    icon: "/gfx/asset_icons/bnb.svg",
  },
  {
    network: sepolia,
    icon: "/gfx/asset_icons/eth.svg",
  },
];
