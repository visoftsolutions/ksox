import { WETH_CONTRACT_ABI } from "~/components/contracts/abi/wethContract";
import { TOKEN_TICKET_CONTRACT_ABI } from "~/components/contracts/abi/tokenTicketContract";
import { Chain, mainnet, polygon, bsc, avalanche, hardhat } from "viem/chains";
import { PHASE_CONTRACT_ABI } from "~/components/contracts/abi/phaseContract";
import { Address } from "viem";

export interface Token {
  icon: string;
  name: string;
  symbol: string;
}

export interface phaseContract {
  address: Address;
  abi: typeof PHASE_CONTRACT_ABI;
}

export interface wethContract {
  address: Address;
  abi: typeof WETH_CONTRACT_ABI;
}

export interface tokenTicketContract {
  address: Address;
  abi: typeof TOKEN_TICKET_CONTRACT_ABI;
}

export interface Network {
  network: Chain;
  icon: string;
  tokens: Token[];
  phaseContract: phaseContract;
  wethContract: wethContract;
  tokenTicketContract: tokenTicketContract;
}

export const AVAILABLE_CHAINS: Network[] = [
  {
    network: mainnet,
    icon: "/gfx/asset_icons/eth.svg",
    phaseContract: {
      address: "0x9A4fb652B552f9e92bc8af063cbb0B8F80690Dc3",
      abi: PHASE_CONTRACT_ABI,
    },
    wethContract: {
      address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
      abi: WETH_CONTRACT_ABI,
    },
    tokenTicketContract: {
      address: "0x7442E41B0a85cdAf8540B69714E46E07ffa4fD6e",
      abi: TOKEN_TICKET_CONTRACT_ABI,
    },
    tokens: [
      { icon: "/gfx/asset_icons/eth.svg", name: "Ethereum", symbol: "ETH" },
      {
        icon: "/gfx/asset_icons/eth.svg",
        name: "Wrapped Ethereum",
        symbol: "WETH",
      },
      { icon: "/gfx/asset_icons/usdc.svg", name: "USD Coin", symbol: "USDC" },
      { icon: "/gfx/asset_icons/usdt.svg", name: "Tether", symbol: "USDT" },
    ],
  },
  {
    network: polygon,
    icon: "/gfx/asset_icons/matic.svg",
    phaseContract: {
      address: "0x9A4fb652B552f9e92bc8af063cbb0B8F80690Dc3",
      abi: PHASE_CONTRACT_ABI,
    },
    wethContract: {
      address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
      abi: WETH_CONTRACT_ABI,
    },
    tokenTicketContract: {
      address: "0x7442E41B0a85cdAf8540B69714E46E07ffa4fD6e",
      abi: TOKEN_TICKET_CONTRACT_ABI,
    },
    tokens: [
      { icon: "/gfx/asset_icons/eth.svg", name: "Ethereum", symbol: "ETH" },
      {
        icon: "/gfx/asset_icons/eth.svg",
        name: "Wrapped Ethereum",
        symbol: "WETH",
      },
      { icon: "/gfx/asset_icons/usdc.svg", name: "USD Coin", symbol: "USDC" },
      { icon: "/gfx/asset_icons/usdt.svg", name: "Tether", symbol: "USDT" },
    ],
  },
  {
    network: bsc,
    icon: "/gfx/asset_icons/bnb.svg",
    phaseContract: {
      address: "0x9A4fb652B552f9e92bc8af063cbb0B8F80690Dc3",
      abi: PHASE_CONTRACT_ABI,
    },
    wethContract: {
      address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
      abi: WETH_CONTRACT_ABI,
    },
    tokenTicketContract: {
      address: "0x7442E41B0a85cdAf8540B69714E46E07ffa4fD6e",
      abi: TOKEN_TICKET_CONTRACT_ABI,
    },
    tokens: [
      { icon: "/gfx/asset_icons/eth.svg", name: "Ethereum", symbol: "ETH" },
      {
        icon: "/gfx/asset_icons/eth.svg",
        name: "Wrapped Ethereum",
        symbol: "WETH",
      },
      { icon: "/gfx/asset_icons/usdc.svg", name: "USD Coin", symbol: "USDC" },
      { icon: "/gfx/asset_icons/usdt.svg", name: "Tether", symbol: "USDT" },
    ],
  },
  {
    network: avalanche,
    icon: "/gfx/asset_icons/avax.svg",
    phaseContract: {
      address: "0x9A4fb652B552f9e92bc8af063cbb0B8F80690Dc3",
      abi: PHASE_CONTRACT_ABI,
    },
    wethContract: {
      address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
      abi: WETH_CONTRACT_ABI,
    },
    tokenTicketContract: {
      address: "0x7442E41B0a85cdAf8540B69714E46E07ffa4fD6e",
      abi: TOKEN_TICKET_CONTRACT_ABI,
    },
    tokens: [
      { icon: "/gfx/asset_icons/eth.svg", name: "Ethereum", symbol: "ETH" },
      {
        icon: "/gfx/asset_icons/eth.svg",
        name: "Wrapped Ethereum",
        symbol: "WETH",
      },
      { icon: "/gfx/asset_icons/usdc.svg", name: "USD Coin", symbol: "USDC" },
      { icon: "/gfx/asset_icons/usdt.svg", name: "Tether", symbol: "USDT" },
    ],
  },
  {
    network: hardhat,
    icon: "/gfx/asset_icons/leo.svg",
    phaseContract: {
      address: "0x9A4fb652B552f9e92bc8af063cbb0B8F80690Dc3",
      abi: PHASE_CONTRACT_ABI,
    },
    wethContract: {
      address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
      abi: WETH_CONTRACT_ABI,
    },
    tokenTicketContract: {
      address: "0x7442E41B0a85cdAf8540B69714E46E07ffa4fD6e",
      abi: TOKEN_TICKET_CONTRACT_ABI,
    },
    tokens: [
      { icon: "/gfx/asset_icons/eth.svg", name: "Ethereum", symbol: "ETH" },
      {
        icon: "/gfx/asset_icons/eth.svg",
        name: "Wrapped Ethereum",
        symbol: "WETH",
      },
      { icon: "/gfx/asset_icons/usdc.svg", name: "USD Coin", symbol: "USDC" },
      { icon: "/gfx/asset_icons/usdt.svg", name: "Tether", symbol: "USDT" },
    ],
  },
];
