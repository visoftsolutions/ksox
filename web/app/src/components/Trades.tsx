import { Index } from "solid-js";
import { createStore } from "solid-js/store";
import { TriElementComponent } from "./TriElement/TriElement";
import TriElement from "./TriElement/TriElement";
import TriElementHeader from "./TriElement/TriElementHeader";
import { init } from "~/memos/Trades";

export interface TradesComponent {
  trades: TriElementComponent[];
}

export const [store, setStore] = createStore<TradesComponent>({ trades: [] });

init();

export default function Trades() {
  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="p-4 font-sanspro text-trades-label font-semibold">Trades</div>
        <TriElementHeader
            class="py-[4px] px-[12px]"
            column_0={<div class="text-left text-trades-sublabel">{"Price (USDT)"}</div>}
            column_1={<div class="text-right text-trades-sublabel">{"Quantity (BTC)"}</div>}
            column_2={<div class="text-right text-trades-sublabel">{"Time"}</div>}
          />
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute top-0 left-0 right-0 bottom-0 flex flex-col overflow-clip">
          <Index each={store.trades}>
            {(element, i) => (
              <TriElement
                class={`py-[4px] px-[12px] font-sanspro text-trades-item ${i % 2 ? "bg-gray-3" : ""}`}
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
