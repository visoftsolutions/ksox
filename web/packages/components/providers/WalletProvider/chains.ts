import { Chain, polygon } from "viem/chains";

export interface Network {
  network: Chain;
  icon: string;
}

export const AVAILABLE_CHAINS: Network[] = [
  {
    network: polygon,
    icon: "/gfx/asset_icons/matic.svg",
  },
];
