import { GetAccountResult, GetNetworkResult, PublicClient } from "@wagmi/core";
import { createContext, JSX, onMount, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { Address, WalletClient, createPublicClient } from "viem";
import {
  EthereumClient,
  w3mConnectors,
  w3mProvider,
} from "@web3modal/ethereum";
import { Web3Modal } from "@web3modal/html";
import { configureChains, createConfig } from "@wagmi/core";
import { createWalletClient, custom } from "viem";
import { AVAILABLE_CHAINS, Network } from "./WalletProvider/chains";
import { logout, setSession } from "./SessionProvider";

export interface WalletProvider {
  walletConnectProjectId: string | undefined;
  selected_network: Network;
  publicClient: PublicClient | undefined;
  walletClient: WalletClient | undefined;
  address: Address | undefined;
}

export const [wallet, setWallet] = createStore<WalletProvider>({
  walletConnectProjectId: undefined,
  selected_network: AVAILABLE_CHAINS[0],
  publicClient: undefined,
  walletClient: undefined,
  address: undefined,
});
const WalletContext = createContext<WalletProvider>(wallet);
export function WalletProvider(props: {
  projectId: string;
  children: JSX.Element;
}) {
  onMount(async () => {
    setWallet({
      walletConnectProjectId: props.projectId,
    });
    await walletClientInit(props.projectId);
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

const walletAccount = async (account: GetAccountResult) => {
  if (account.address && account.connector && account.isConnected) {
    const publicClient = createPublicClient({
      chain: wallet.selected_network.network,
      transport: custom(await account.connector.getProvider()),
    });
    const walletClient = createWalletClient({
      chain: wallet.selected_network.network,
      transport: custom(await account.connector.getProvider()),
    });
    setWallet({ publicClient, walletClient, address: account.address });
  } else {
    setWallet({
      publicClient: undefined,
      walletClient: undefined,
      address: undefined,
    });
    setSession(undefined);
  }
};

const walletNetwork = async (network: GetNetworkResult) => {
  const net = AVAILABLE_CHAINS.find((e) => e.network.id == network.chain?.id);
  if (net != undefined) {
    setWallet({ selected_network: net });
  }
};

export const walletClientInit = async (projectId: string) => {
  const { publicClient, chains } = configureChains(
    AVAILABLE_CHAINS.map((e) => e.network),
    [w3mProvider({ projectId })],
  );
  const wagmiConfig = createConfig({
    autoConnect: true,
    connectors: w3mConnectors({
      projectId,
      chains,
    }),
    publicClient,
  });
  const ethereumClient = new EthereumClient(wagmiConfig, chains);

  ethereumClient.watchAccount(async (account) => await walletAccount(account));
  ethereumClient.watchNetwork(async (network) => await walletNetwork(network));

  return ethereumClient;
};

export const walletClientConnect = async () => {
  if (wallet.walletConnectProjectId) {
    const ethereumClient = await walletClientInit(
      wallet.walletConnectProjectId,
    );
    const web3modal = new Web3Modal(
      { projectId: wallet.walletConnectProjectId },
      ethereumClient,
    );
    await web3modal.openModal();
  }
};
