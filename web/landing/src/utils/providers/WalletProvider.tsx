import { PublicClient } from "@wagmi/core";
import { createContext, JSX, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { Address, CustomTransport, http, HttpTransport, WalletClient, webSocket, WebSocketTransport } from "viem";
import {
  Chain,
  mainnet,
  polygon,
  bsc,
  avalanche,
  localhost,
} from "viem/chains";
import { phaseContractAbi } from "~/components/contract";
import {
  EthereumClient,
  w3mConnectors,
  w3mProvider,
} from "@web3modal/ethereum";
import { Web3Modal } from "@web3modal/html";
import { configureChains, createConfig } from "@wagmi/core";
import { createPublicClient, createWalletClient, custom } from "viem";

export interface Token {
  icon: string;
  name: string;
  symbol: string;
}

export interface Contract {
  address: Address;
  abi: typeof phaseContractAbi;
}

export interface Network {
  network: Chain;
  icon: string;
  tokens: Token[];
  contract: Contract;
}

export const availableChains: Network[] = [
  {
    network: mainnet,
    icon: "/gfx/asset_icons/eth.svg",
    contract: {
      address: "0xc0c5618f0F3Fa66b496F2940f373DC366d765BAe",
      abi: phaseContractAbi,
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
    contract: {
      address: "0xc0c5618f0F3Fa66b496F2940f373DC366d765BAe",
      abi: phaseContractAbi,
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
    contract: {
      address: "0xc0c5618f0F3Fa66b496F2940f373DC366d765BAe",
      abi: phaseContractAbi,
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
    contract: {
      address: "0xc0c5618f0F3Fa66b496F2940f373DC366d765BAe",
      abi: phaseContractAbi,
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
    network: localhost,
    icon: "/gfx/asset_icons/leo.svg",
    contract: {
      address: "0xc0c5618f0F3Fa66b496F2940f373DC366d765BAe",
      abi: phaseContractAbi,
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

export interface WalletProvider {
  selected_network: Network;
  selected_token: Token;
  walletClient: WalletClient<CustomTransport, typeof localhost> | undefined;
  publicClient: PublicClient<HttpTransport, typeof localhost>;
  publicWSClient: PublicClient<WebSocketTransport, typeof localhost>;
  address: Address | undefined;
}

export const [wallet, setWallet] = createStore<WalletProvider>({
  walletClient: undefined,
  publicClient: createPublicClient({
    chain: localhost,
    transport: http("http://127.0.0.1:8545/"),
  }),
  publicWSClient: createPublicClient({
    chain: localhost,
    transport: webSocket("ws://127.0.0.1:8545/"),
  }),
  address: undefined,
  selected_network: availableChains[0],
  selected_token: availableChains[0].tokens[0],
});
const WalletContext = createContext<WalletProvider>(wallet);
export function WalletProvider(props: { children: JSX.Element }) {
  return (
    <WalletContext.Provider value={wallet}>
      {props.children}
    </WalletContext.Provider>
  );
}
export function useWallet() {
  return useContext<WalletProvider>(WalletContext);
}

const projectId = import.meta.env.VITE_WALLET_CONNECT_PROJECT_ID;

export const walletConnect = async () => {
  const { publicClient, chains } = configureChains(availableChains.map((e) => e.network), [
    w3mProvider({ projectId }),
  ]);
  const wagmiConfig = createConfig({
    autoConnect: true,
    connectors: w3mConnectors({ projectId, version: 1, chains }),
    publicClient
  });
  const ethereumClient = new EthereumClient(wagmiConfig, chains);
  const web3modal = new Web3Modal({ projectId }, ethereumClient);
  await web3modal.openModal();

  ethereumClient.watchAccount(async (account) => {
    if (account.address && account.connector && account.isConnected) {
      const walletClient = createWalletClient({
        chain: localhost,
        transport: custom(await account.connector.getProvider()),
      });
      setWallet({ walletClient: walletClient });

      const address = await walletClient.getAddresses().then((e) => e.at(0));
      setWallet({ address: address });
    } else {
      setWallet({ 
        walletClient: undefined,
        address: undefined
      });
    }
  });

  ethereumClient.watchNetwork(async (network) => {
    setWallet({
      selected_network: availableChains.find(
        (e) => e.network.id == network.chain?.id
      ),
    });
  });
};
