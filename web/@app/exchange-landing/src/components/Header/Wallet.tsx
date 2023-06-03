import NetworkDropdown from "../Token/NetworkDropdown";
import { useWallet, walletClientConnect } from "@web/components/providers/WalletProvider";
import { session, setSession, login, logout } from "@web/components/providers/SessionProvider";
import firstLastChars from "@web/utils/firstLastChars";
import { api, projectId } from "~/root";

export default function Wallet(props: { class: string }) {
  const wallet = useWallet();

  return (
    <div class={`grid grid-cols-[auto_auto] items-center justify-center gap-4 ${props.class}`}>
      <NetworkDropdown disabled={false} />
      <div
        class="token-linear-wipe-button cursor-pointer rounded-full px-4 py-2 text-center font-lexend font-medium text-text-1"
        onClick={async () => {
          if (!wallet.walletClient) {
            await walletClientConnect(projectId)
          }
        }}
      >
        {wallet.address == undefined ? "Connect" : firstLastChars(wallet.address, 6, 6)}
      </div>
    </div>
  );
}
