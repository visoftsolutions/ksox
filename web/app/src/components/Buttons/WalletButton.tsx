import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import { createContext, createSignal, Signal, useContext } from "solid-js";
import { EthereumClient, w3mConnectors, w3mProvider } from "@web3modal/ethereum";
import { Web3Modal } from "@web3modal/html";
import { configureChains, createClient } from "@wagmi/core";
import { mainnet, polygon } from "@wagmi/core/chains";
import { createWalletClient, custom, CustomTransport, WalletClient } from "viem";
import { ValidateSignatureResponse } from "~/auth/mod";
import login from "~/auth/login";
import { JSX } from "solid-js/web/types/jsx";

const projectId = import.meta.env.VITE_WALLET_CONNECT_PROJECT_ID;

export const WalletContext = createContext<Signal<WalletClient<CustomTransport, typeof mainnet> | null>>();
export function WalletProvider(props: { children: JSX.Element }) {
  return (
    <WalletContext.Provider value={createSignal<WalletClient<CustomTransport, typeof mainnet> | null>(null)}>
      {props.children}
    </WalletContext.Provider>
  );
}

export const SessionContext = createContext<Signal<ValidateSignatureResponse | null>>();
export function LoginProvider(props: { children: JSX.Element }) {
  return (
    <SessionContext.Provider value={createSignal<ValidateSignatureResponse | null>(null)}>
      {props.children}
    </SessionContext.Provider>
  );
}

export default function WalletButton() {
  const [wallet, setWallet] = useContext(WalletContext)!;
  const [session, setSession] = useContext(SessionContext)!;

  const walletConnect = async () => {
    const BASE_URL = location.pathname;
    const API_URL = joinPaths(BASE_URL, "/api");

    const { chains, provider, webSocketProvider } = configureChains([mainnet, polygon], [w3mProvider({ projectId })]);

    const client = createClient({
      autoConnect: true,
      connectors: w3mConnectors({ projectId, version: 1, chains }),
      provider,
      webSocketProvider,
    });
    const ethereumClient = new EthereumClient(client, chains);
    const web3modal = new Web3Modal({ projectId }, ethereumClient);
    await web3modal.openModal();

    ethereumClient.watchAccount(async (account) => {
      if (account.address && account.connector) {
        setWallet(createWalletClient({
          chain: mainnet,
          transport: custom(await account.connector.getProvider()),
        }));
      }
    });
  };

  return (
    <div
      class="grid cursor-pointer select-none grid-cols-[auto_auto] grid-rows-[1fr] items-center justify-center gap-4 px-4"
      onClick={async () => {
        const w = wallet();
        w ? setSession(await login(w)) : await walletConnect()
      }}
    >
      <div class="text-mainmenu-wallet font-normal">{
        !wallet() ? "CONNECT WALLET" : !session() ? "LOGIN" : session()?.user_id
      }</div>
      <img src={joinPaths(base, "gfx/metamask.webp")} class="m-auto w-[22px]" />
    </div>
  );
}
