import SearchBar from "~/components/Atoms/SearchBar";
import WalletButton from "~/components/Atoms/Buttons/WalletButton";
import { api, base } from "~/root";
import { Setter } from "solid-js";
import ColorModeButton from "./Atoms/Buttons/ColorModeButton";

export default function Header(props: {
  class?: string;
  text?: string;
  setSearch: Setter<string | undefined>;
}) {
  return (
    <div
      class={`grid grid-cols-[1fr_auto] xl:grid-cols-1 grid-rows-[3rem_auto] items-center justify-center gap-4 ${props.class}`}
    >
      <div class="text-3xl font-sans font-bold text-r-light-text dark:text-r-dark-text row-start-1 row-end-2 col-start-1 col-end-2">
        {props.text}
      </div>
      <div class="justify-self-end row-start-1 row-end-2 col-start-2 col-end-3 grid grid-cols-[auto-auto] justify-end items-center justify-items-center gap-2">
        <div class="col-start-1 col-end-2">
          <ColorModeButton class="xl:hidden" />
        </div>
        <div class="col-start-2 col-end-3">
          <WalletButton base_url={base} api_url={api} />
        </div>
      </div>
      <div class="row-start-2 row-end-3 col-span-2">
        <SearchBar
          placeholder="Search"
          onInput={(input) => {
            props.setSearch(input);
          }}
        />
      </div>
    </div>
  );
}
