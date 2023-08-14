import { Chain, hardhat } from "viem/chains";

export interface Network {
  network: Chain;
  icon: string;
}

export const AVAILABLE_CHAINS: Network[] = [
  {
    network: hardhat,
    icon: "/gfx/asset_icons/eth.svg",
  },
];
