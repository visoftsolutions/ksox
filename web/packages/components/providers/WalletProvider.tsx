import {
  GetAccountResult,
  GetNetworkResult,
  PublicClient,
  sepolia,
} from "@wagmi/core";
import { createContext, JSX, onMount, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import {
  Address,
  CustomTransport,
  http,
  HttpTransport,
  WalletClient,
  webSocket,
  WebSocketTransport,
} from "viem";
import {
  EthereumClient,
  w3mConnectors,
  w3mProvider,
} from "@web3modal/ethereum";
import { Web3Modal } from "@web3modal/html";
import { configureChains, createConfig } from "@wagmi/core";
import { createPublicClient, createWalletClient, custom } from "viem";
import { AVAILABLE_CHAINS, Network } from "./WalletProvider/chains";

export interface WalletProvider {
  selected_network: Network;
  walletConnectProjectId: string | undefined;
  walletClient: WalletClient<CustomTransport, typeof sepolia> | undefined;
  publicClient: PublicClient<HttpTransport, typeof sepolia> | undefined;
  publicWSClient: PublicClient<WebSocketTransport, typeof sepolia> | undefined;
  address: Address | undefined;
}

export const [wallet, setWallet] = createStore<WalletProvider>({
  walletClient: undefined,
  address: undefined,
  walletConnectProjectId: undefined,
  publicClient: undefined,
  publicWSClient: undefined,
  selected_network: AVAILABLE_CHAINS[0],
});
const WalletContext = createContext<WalletProvider>(wallet);
export function WalletProvider(props: {
  children: JSX.Element;
  projectId: string;
  alchemyId: string;
}) {
  onMount(() => {
    setWallet({
      walletConnectProjectId: props.projectId,
      publicClient: createPublicClient({
        // chain: hardhat,
        chain: sepolia,
        // transport: http("http://127.0.0.1:8545/"),
        transport: http(
          `https://eth-sepolia.g.alchemy.com/v2/${props.alchemyId}`,
        ),
      }),
      publicWSClient: createPublicClient({
        // chain: hardhat,
        chain: sepolia,
        // transport: webSocket("ws://127.0.0.1:8545/"),
        transport: webSocket(
          `wss://eth-sepolia.g.alchemy.com/v2/${props.alchemyId}`,
        ),
      }),
    });
  });

  return (
    <WalletContext.Provider value={wallet}>
      {props.children}
    </WalletContext.Provider>
  );
}
export function useWallet() {
  return useContext<WalletProvider>(WalletContext);
}

const walletAccount = async (account: GetAccountResult<PublicClient>) => {
  if (account.address && account.connector && account.isConnected) {
    const walletClient = createWalletClient({
      chain: sepolia,
      transport: custom(await account.connector.getProvider()),
    });
    setWallet({ walletClient: walletClient, address: account.address });
  } else {
    setWallet({ walletClient: undefined, address: undefined });
  }
};

const walletNetwork = async (network: GetNetworkResult) => {
  const net = AVAILABLE_CHAINS.find((e) => e.network.id == network.chain?.id);
  if (net != undefined) {
    setWallet({ selected_network: net });
  }
};

export const walletClientConnect = async () => {
  if (wallet.walletConnectProjectId) {
    const { publicClient, chains } = configureChains(
      AVAILABLE_CHAINS.map((e) => e.network),
      [w3mProvider({ projectId: wallet.walletConnectProjectId })],
    );
    const wagmiConfig = createConfig({
      autoConnect: true,
      connectors: w3mConnectors({
        projectId: wallet.walletConnectProjectId,
        chains,
      }),
      publicClient,
    });
    const ethereumClient = new EthereumClient(wagmiConfig, chains);
    const web3modal = new Web3Modal(
      { projectId: wallet.walletConnectProjectId },
      ethereumClient,
    );
    ethereumClient.watchAccount(
      async (account) => await walletAccount(account),
    );
    ethereumClient.watchNetwork(
      async (network) => await walletNetwork(network),
    );
    await web3modal.openModal();
  }
};
