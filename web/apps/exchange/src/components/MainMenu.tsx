import WalletButton from "~/components/MainMenu/WalletButton";
import { api, base } from "~/root";

export default function MainMenu() {
  return (
    <div class="justify-center, grid h-full grid-cols-[1fr_auto] grid-rows-[1fr] items-center px-3">
      <div class="col-start-2 col-end-3">
        <WalletButton base_url={base} api_url={api} />
      </div>
    </div>
  );
}
