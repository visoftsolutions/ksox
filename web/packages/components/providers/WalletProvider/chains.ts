import { Chain, mainnet, polygon, bsc } from "viem/chains";
import { Address } from "viem";

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
];
