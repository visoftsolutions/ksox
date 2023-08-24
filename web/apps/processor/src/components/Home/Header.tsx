import NotificationsButton from "~/components/Atoms/Buttons/NotificationsButton";
import Picture from "~/components/Atoms/Picture";
import SearchBar from "~/components/Atoms/SearchBar";
import {
  ColorMode,
  useColorMode,
} from "@packages/components/providers/ColorModeProvider";
import NewButton from "~/components/Atoms/Buttons/NewButton";
import Wallet from "../Wallet";
import WalletButton from "../Wallet/WalletButton";
import { api, base } from "~/root";

export default function Header(props: { class?: string }) {
  const colorMode = useColorMode();
  return (
    <div
      class={`grid grid-cols-[1fr_auto] grid-rows-[auto_auto] justify-center gap-4 ${props.class}`}
    >
      <div class="text-3xl font-sans font-bold text-r-light-text dark:text-r-dark-text row-start-1 row-end-2 col-start-1 col-end-2">
        Home
      </div>
      <div class="justify-self-end row-start-1 row-end-2 col-start-2 col-end-3 xl:hidden">
        <WalletButton base_url={base} api_url={api} />
      </div>
      <div class="row-start-2 row-end-3 col-span-2">
        <SearchBar placeholder="Search" />
      </div>
    </div>
  );
}
