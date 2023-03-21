import { Index } from "solid-js";
import { createStore } from "solid-js/store";
import SearchInput from "./Inputs/SearchInput";
import TriElement, { TriElementDisplay } from "./TriElement/TriElement";
import TriElementHeader from "./TriElement/TriElementHeader";

export interface MarketsDisplay {
  search: string;
  markets: TriElementDisplay[];
}

const [store, setStore] = createStore<MarketsDisplay>({
  search: "",
  markets: [],
});

setStore({
  search: "",
  markets: [
    {
      columns: [<div class="text-left">{"MONA/USDT"}</div>, "0.002234", <span class="font-semibold text-green">{"12.34%"}</span>],
    },
    {
      columns: [<div class="text-left">{"MONA/USDT"}</div>, "0.002234", <span class="font-semibold text-green">{"12.34%"}</span>],
    },
    {
      columns: [<div class="text-left">{"MONA/USDT"}</div>, "0.002234", <span class="font-semibold text-green">{"12.34%"}</span>],
    },
    {
      columns: [<div class="text-left">{"MONA/USDT"}</div>, "0.002234", <span class="font-semibold text-green">{"12.34%"}</span>],
    },
    {
      columns: [<div class="text-left">{"MONA/USDT"}</div>, "0.002234", <span class="font-semibold text-green">{"12.34%"}</span>],
    },
    {
      columns: [<div class="text-left">{"MONA/USDT"}</div>, "0.002234", <span class="font-semibold text-green">{"12.34%"}</span>],
    },
    {
      columns: [<div class="text-left">{"MONA/USDT"}</div>, "0.002234", <span class="font-semibold text-green">{"12.34%"}</span>],
    },
    {
      columns: [<div class="text-left">{"MONA/USDT"}</div>, "0.002234", <span class="font-semibold text-green">{"12.34%"}</span>],
    },
  ],
});

export default function Markets() {
  return (
    <div class="grid grid-cols-1 grid-rows-[auto_1fr]">
      <div class="row-start-1 row-end-2 ">
        <div class="p-4 font-sanspro text-trades-label font-semibold">Markets</div>
        <div class="px-[12px]">
          <SearchInput
            class="mx-auto mb-2 w-full text-markets-searchbar"
            left={
              <>
                <img src="gfx/search.svg" />
              </>
            }
          />
          <TriElementHeader
            columns={[
              <div class="text-left text-markets-sublabel">{"Market"}</div>,
              <div class="text-right text-markets-sublabel">{"Price (USDT)"}</div>,
              <div class="text-right text-markets-sublabel">{"Change"}</div>,
            ]}
          />
        </div>
      </div>
      <div class="row-start-2 row-end-3 ">
        <Index each={store.markets}>
          {(element, i) => (
            <TriElement
              class={`px-[12px] py-2 text-right font-sanspro text-markets-item ${i % 2 ? "bg-gray-3" : ""}`}
              columns={[element().columns[0], element().columns[1], element().columns[2]]}
            />
          )}
        </Index>
      </div>
    </div>
  );
}
