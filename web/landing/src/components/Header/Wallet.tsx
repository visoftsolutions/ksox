import { DefaultProps } from "~/utils/interfaces";
import NetworkDropdown from "../Token/NetworkDropdown";
import { firstLastChars } from "~/utils/formatters/AddressFormatter";
import { useWallet, walletConnect } from "~/utils/providers/WalletProvider";

export default function Wallet(props: DefaultProps) {
  const wallet = useWallet();

  return (
    <div class={`grid grid-cols-[auto_auto] gap-4 ${props.class}`}>
      <NetworkDropdown disabled={false} />
      <div
        class="token-linear-wipe-button cursor-pointer rounded-full px-4 py-2 text-center font-lexend font-medium text-text-1"
        onClick={async () => {
          await walletConnect();
        }}
      >
        {wallet.address == undefined
          ? "Connect"
          : firstLastChars(wallet.address, 6, 4)}
      </div>
    </div>
  );
}