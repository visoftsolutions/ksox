import SearchBar from "~/components/Atoms/SearchBar";
import WalletButton from "../Atoms/Buttons/WalletButton";
import { api, base } from "~/root";

export default function Header(props: { class?: string }) {
  return (
    <div
      class={`grid grid-cols-[1fr_auto] xl:grid-cols-1 grid-rows-[3rem_auto] items-center justify-center gap-4 ${props.class}`}
    >
      <div class="text-3xl font-sans font-bold text-r-light-text dark:text-r-dark-text row-start-1 row-end-2 col-start-1 col-end-2">
        Home
      </div>
      <div class="justify-self-end row-start-1 row-end-2 col-start-2 col-end-3 grid">
        <WalletButton base_url={base} api_url={api} class="justify-self-end" />
      </div>
      <div class="row-start-2 row-end-3 col-span-2">
        <SearchBar placeholder="Search" />
      </div>
    </div>
  );
}
