import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import { Accessor, createContext, createSignal, useContext } from "solid-js";
import { EthereumClient, w3mConnectors, w3mProvider } from "@web3modal/ethereum";
import { Web3Modal } from "@web3modal/html";
import { configureChains, createClient } from "@wagmi/core";
import { mainnet, polygon } from "@wagmi/core/chains";
import { createWalletClient, custom, CustomTransport, WalletClient } from "viem";
import { ValidateSignatureResponse } from "~/auth/mod";
import login from "~/auth/login";
import { JSX } from "solid-js/web/types/jsx";
import logout from "~/auth/logout";

const projectId = import.meta.env.VITE_WALLET_CONNECT_PROJECT_ID;

const [wallet, setWallet] = createSignal<WalletClient<CustomTransport, typeof mainnet> | null>(null);
const WalletContext = createContext<Accessor<WalletClient<CustomTransport, typeof mainnet> | null>>(wallet);
export function WalletProvider(props: { children: JSX.Element }) {
  return <WalletContext.Provider value={wallet}>{props.children}</WalletContext.Provider>;
}
export function useWallet() {
  return useContext<Accessor<WalletClient<CustomTransport, typeof mainnet> | null>>(WalletContext);
}

const [session, setSession] = createSignal<ValidateSignatureResponse | null>(null);
const SessionContext = createContext<Accessor<ValidateSignatureResponse | null>>(session);
export function SessionProvider(props: { children: JSX.Element }) {
  return <SessionContext.Provider value={session}>{props.children}</SessionContext.Provider>;
}
export function useSession() {
  return useContext<Accessor<ValidateSignatureResponse | null>>(SessionContext);
}

export default function WalletButton() {
  const walletConnect = async () => {
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
        setWallet(
          createWalletClient({
            chain: mainnet,
            transport: custom(await account.connector.getProvider()),
          })
        );
        setSession(null);
      }
    });
  };

  return (
    <div
      class="grid cursor-pointer select-none grid-cols-[auto_auto] grid-rows-[1fr] items-center justify-center gap-4 px-4"
      onClick={async () => {
        const w = wallet();
        const s = session();
        !w ? await walletConnect() : !s ? setSession(await login(w)) : setSession(await logout());
      }}
    >
      <div class="text-mainmenu-wallet font-normal">{!wallet() ? "CONNECT WALLET" : !session() ? "LOGIN" : "LOGOUT"}</div>
      <img src={joinPaths(base, "gfx/metamask.webp")} class="m-auto w-[22px]" />
    </div>
  );
}
