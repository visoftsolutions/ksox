import { Chain, localhost } from "viem/chains";

export interface Network {
  network: Chain;
  icon: string;
}

export const AVAILABLE_CHAINS: Network[] = [
  {
    network: {
      ...localhost,
      id: 31337,
    },
    icon: "/gfx/asset_icons/eth.svg",
  },
];
