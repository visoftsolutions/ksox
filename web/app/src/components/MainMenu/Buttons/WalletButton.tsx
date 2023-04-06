import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import { EthereumClient, w3mConnectors, w3mProvider } from "@web3modal/ethereum";
import { Web3Modal } from "@web3modal/html";
import { configureChains, createClient } from "@wagmi/core";
import { mainnet, polygon } from "@wagmi/core/chains";
import { createWalletClient, custom } from "viem";
import login from "~/auth/login";
import logout from "~/auth/logout";
import { setWallet, wallet } from "~/utils/providers/WalletProvider";
import { session, setSession } from "~/utils/providers/SessionProvider";
import { firstLastChars } from "~/utils/formatters/AddressFormatter";

const projectId = import.meta.env.VITE_WALLET_CONNECT_PROJECT_ID;

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
        setSession(undefined);
      }
    });
  };

  return (
    <div
      class={`grid cursor-pointer select-none h-[32px] grid-cols-[auto_1fr] items-center justify-center gap-2 px-5 rounded-[40px] ${!wallet() || !session()  ? "bg-ksox-1" : "bg-gray-3"}`}
      onClick={async () => {
        const w = wallet();
        const s = session();
        !w ? await walletConnect() : !s ? setSession(await login(w)) : setSession(await logout());
      }}
    >
      { !session() ? 
      <img src={joinPaths(base, "gfx/wallet.svg")} alt="wallet" width="22px" class="m-auto" />
      : <img src={joinPaths(base, "gfx/user.svg")} alt="user" width="16px" class="m-auto" />}
      <div class="text-wallet font-semibold text-ellipsis">{!wallet() ? "CONNECT WALLET" : !session() ? "LOGIN" : session()?.user_id}</div>
    </div>
  );
}
