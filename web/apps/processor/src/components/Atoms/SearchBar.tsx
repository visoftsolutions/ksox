import { createSignal } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export interface ISearchBar {
  placeholder?: string;
  onInput: (input: string) => void;
}

export default function SearchBar(props: ISearchBar) {
  return (
    <div class="grid grid-flow-col grid-cols-[auto_1fr] rounded-full bg-r-light-search-bar dark:bg-r-dark-search-bar">
      <img
        src={joinPaths(base, "gfx/search.svg")}
        alt="Search Icon"
        class="w-[18px] h-[18px] m-2 ml-3 p-0"
      />
      <input
        type="text"
        onInput={(e) => props.onInput(e.target.value)}
        class="flex-grow rounded-r-full px-1 py-1 outline-none bg-r-light-search-bar dark:bg-r-dark-search-bar m-0 p-0 text-[14px]"
        placeholder={props.placeholder || "Search"}
      />
    </div>
  );
}
