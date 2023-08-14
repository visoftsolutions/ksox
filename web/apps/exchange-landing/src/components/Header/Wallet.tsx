import NetworkDropdown from "~/components/Token/NetworkDropdown";
import {
  useWallet,
  walletClientConnect,
} from "@packages/components/providers/WalletProvider";
import firstLastChars from "@packages/utils/firstLastChars";

export default function Wallet(props: { class: string }) {
  const wallet = useWallet();

  return (
    <div
      class={`grid grid-cols-[auto_auto] items-center justify-center gap-4 ${props.class}`}
    >
      <NetworkDropdown disabled={false} />
      <div
        class="token-linear-wipe-button cursor-pointer rounded-full px-4 py-2 text-center font-lexend font-medium text-text-1"
        onClick={async () => {
          if (!wallet.walletClient) {
            await walletClientConnect();
          }
        }}
      >
        {wallet.address == undefined
          ? "Connect"
          : firstLastChars(wallet.address, 6, 6)}
      </div>
    </div>
  );
}
