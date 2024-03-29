import { Index } from "solid-js";
import { createStore } from "solid-js/store";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import SearchInput from "~/components/Inputs/SearchInput";
import TriElementHeader from "~/components/App/TriElement/TriElementHeader";
import TriElement, {
  TriElementComponent,
} from "~/components/App/TriElement/TriElement";

export interface MarketsComponent {
  search: string;
  markets: TriElementComponent[];
}

export const [store, setStore] = createStore<MarketsComponent>({
  search: "",
  markets: [],
});

export default function Markets() {
  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="p-4 font-sanspro text-trades-label font-semibold">
          Markets
        </div>
        <div class="px-[12px]">
          <SearchInput
            class="text-markets-searchbar mx-auto mb-2 w-full"
            left={
              <>
                <img src={joinPaths(base, "gfx/search.svg")} />
              </>
            }
          />
          <TriElementHeader
            column_0={
              <div class="text-left text-markets-sublabel">{"Market"}</div>
            }
            column_1={
              <div class="text-right text-markets-sublabel">
                {"Price (USDT)"}
              </div>
            }
            column_2={
              <div class="text-right text-markets-sublabel">{"Change"}</div>
            }
          />
        </div>
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col overflow-clip">
          <Index each={store.markets}>
            {(element, i) => (
              <TriElement
                class={`px-[12px] py-2 text-right font-sanspro text-markets-item ${
                  i % 2 ? "bg-gray-3" : ""
                }`}
                column_0={element().column_0}
                column_1={element().column_1}
                column_2={element().column_2}
              />
            )}
          </Index>
        </div>
      </div>
    </div>
  );
}
