import { GetAccountResult, GetNetworkResult, PublicClient } from "@wagmi/core";
import { createContext, JSX, useContext } from "solid-js";
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
import { hardhat, mainnet } from "viem/chains";
import {
  EthereumClient,
  w3mConnectors,
  w3mProvider,
} from "@web3modal/ethereum";
import { Web3Modal } from "@web3modal/html";
import { configureChains, createConfig } from "@wagmi/core";
import { createPublicClient, createWalletClient, custom } from "viem";
import {
  AVAILABLE_CHAINS,
  Network,
  Token,
} from "~/components/contracts/chains";

export interface WalletProvider {
  selected_network: Network;
  selected_token: Token;
  walletClient: WalletClient<CustomTransport, typeof mainnet> | undefined;
  publicClient: PublicClient<HttpTransport, typeof mainnet>;
  publicWSClient: PublicClient<WebSocketTransport, typeof mainnet>;
  address: Address | undefined;
}

export const [wallet, setWallet] = createStore<WalletProvider>({
  walletClient: undefined,
  address: undefined,
  publicClient: createPublicClient({
    // chain: hardhat,
    chain: mainnet,
    // transport: http("http://127.0.0.1:8545/"),
    transport: http("https://eth-goerli.g.alchemy.com/v2/YBzQbzNel58NfEmy574HdQ2hPKjfO93g"),
  }),
  publicWSClient: createPublicClient({
    // chain: hardhat,
    chain: mainnet,
    // transport: webSocket("ws://127.0.0.1:8545/"),
    transport: webSocket("wss://eth-goerli.g.alchemy.com/v2/YBzQbzNel58NfEmy574HdQ2hPKjfO93g"),
  }),
  selected_network: AVAILABLE_CHAINS[0],
  selected_token: AVAILABLE_CHAINS[0].tokens[0],
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

const walletAccount = async (account: GetAccountResult<PublicClient>) => {
  if (account.address && account.connector && account.isConnected) {
    const walletClient = createWalletClient({
      chain: mainnet,
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
  const { publicClient, chains } = configureChains(
    AVAILABLE_CHAINS.map((e) => e.network),
    [w3mProvider({ projectId })]
  );
  const wagmiConfig = createConfig({
    autoConnect: true,
    connectors: w3mConnectors({ projectId, version: 1, chains }),
    publicClient,
  });
  const ethereumClient = new EthereumClient(wagmiConfig, chains);
  const web3modal = new Web3Modal({ projectId }, ethereumClient);
  await web3modal.openModal();

  ethereumClient.watchAccount(async (account) => await walletAccount(account));
  ethereumClient.watchNetwork(async (network) => await walletNetwork(network));
};
