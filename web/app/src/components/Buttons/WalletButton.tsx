import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import { Accessor, createContext, Setter, useContext } from "solid-js";
import { EthereumClient, w3mConnectors, w3mProvider } from "@web3modal/ethereum";
import { Web3Modal } from "@web3modal/html";
import { configureChains, createClient } from "@wagmi/core";
import { mainnet, polygon } from "@wagmi/core/chains";
import { createWalletClient, custom, CustomTransport, getAccount, WalletClient } from "viem";
import { GenerateNonceRequest, GenerateNonceResponse, ValidateSignatureRequest, ValidateSignatureResponse } from "~/auth/mod";
import params from "~/utils/params";

const projectId = import.meta.env.VITE_WALLET_CONNECT_PROJECT_ID;

export const UserWallet = createContext([() => null, () => {}] as [
  Accessor<WalletClient<CustomTransport, typeof mainnet> | null>,
  Setter<WalletClient<CustomTransport, typeof mainnet> | null>
]);

export default function WalletButton() {
  const [wallet, setWallet] = useContext(UserWallet);

  const walletConnect = async () => {
    const BASE_URL = location.pathname;
    const API_URL = joinPaths(BASE_URL, "/api");
    const AUTH_URL = joinPaths(API_URL, "/auth");

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
        const wallet = createWalletClient({
          chain: mainnet,
          transport: custom(await account.connector.getProvider()),
        });

        const generateNonceResponse = await fetch(
          `${AUTH_URL}?${params(
            GenerateNonceRequest.parse({
              address: account.address,
            })
          )}`,
          {
            method: "GET",
          }
        )
          .then((r) => r.json())
          .then((r) => GenerateNonceResponse.parse(r));

        console.log(generateNonceResponse);

        const signature = await wallet.signMessage({
          account: getAccount(account.address),
          message: generateNonceResponse.nonce,
        });

        const validateSignatureResponse = await fetch(`${AUTH_URL}`, {
          method: "POST",
          headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
          },
          body: JSON.stringify(
            ValidateSignatureRequest.parse({
              address: account.address,
              signature,
            })
          ),
        })
          .then((r) => r.json())
          .then((r) => ValidateSignatureResponse.parse(r));
        console.log(validateSignatureResponse);
        setWallet(wallet);
      }
    });
  };
  const walletDisconnect = async () => {
    setWallet(null);
  };

  return (
    <div
      class="grid cursor-pointer select-none grid-cols-[auto_auto] grid-rows-[1fr] items-center justify-center gap-4 px-4"
      onClick={async () => {
        wallet() ? await walletDisconnect() : await walletConnect();
      }}
    >
      <div class="text-mainmenu-wallet font-normal">{wallet() ? "DISCONNECT" : "CONNECT"} WALLET</div>
      <img src={joinPaths(base, "gfx/metamask.webp")} class="m-auto w-[22px]" />
    </div>
  );
}
