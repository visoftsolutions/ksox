import { api, base } from "~/root";
import WalletButton from "./Wallet/WalletButton";

export default function Wallet() {
  return (
    <div class="grid grid-flow-row gap-1 items-center">
      <WalletButton base_url={base} api_url={api} class="justify-self-end" />
    </div>
  );
}
