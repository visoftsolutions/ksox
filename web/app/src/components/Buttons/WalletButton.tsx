import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import { UserSession } from "~/routes/(app)";
import { useContext } from "solid-js";

const projectId = import.meta.env.VITE_WALLET_CONNECT_PROJECT_ID;

export default function WalletButton() {
  const [userSession, setUserSession] = useContext(UserSession);

  // const walletConnect = async () => {
  //   const web3Modal = new Web3Modal({
  //     walletConnectVersion: 2,
  //     projectId,
  //     standaloneChains: ["eip155:1"],
  //   });

  //   let signClient = await SignClient.init({ projectId });

  //   const { uri, approval } = await signClient.connect({
  //     requiredNamespaces: {
  //       eip155: {
  //         methods: ["eth_sign"],
  //         chains: ["eip155:1"],
  //         events: ["accountsChanged"],
  //       },
  //     },
  //   });

  //   if (uri) {
  //     web3Modal.openModal({ uri, standaloneChains: ["eip155:1"] });
  //     let sessionNamespace = await approval();
  //     console.log(sessionNamespace);
  //     web3Modal.closeModal();
  //   }
  // };

  return (
    <div class="grid cursor-pointer select-none grid-cols-[auto_auto] grid-rows-[1fr] items-center justify-center gap-4 px-4" onClick={async () => {}}>
      <div class="text-mainmenu-wallet font-normal">CONNECT WALLET</div>
      <img src={joinPaths(base, "gfx/metamask.webp")} class="m-auto w-[22px]" />
    </div>
  );
}
