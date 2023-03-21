import { Index } from "solid-js";
import { createStore } from "solid-js/store";
import { TriElementDisplay } from "./TriElement/TriElement";
import TriElement from "./TriElement/TriElement";
import TriElementHeader from "./TriElement/TriElementHeader";

export interface TradesDisplay {
  trades: TriElementDisplay[];
}

const [store, setStore] = createStore<TradesDisplay>({ trades: [] });

setStore({
  trades: [
    {
      columns: [<span class="text-green">{"22,342.12"}</span>, "0.002234", "0.002234"],
    },
    {
      columns: [<span class="text-green">{"22,342.12"}</span>, "0.002234", "0.002234"],
    },
    {
      columns: [<span class="text-red">{"22,342.12"}</span>, "0.002234", "0.002234"],
    },
    {
      columns: [<span class="text-red">{"22,342.12"}</span>, "0.002234", "0.002234"],
    },
    {
      columns: [<span class="text-green">{"22,342.12"}</span>, "0.002234", "0.002234"],
    },
  ],
});

export default function Trades() {
  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="p-4 font-sanspro text-trades-label font-semibold">Trades</div>
        <div class="pr-[12px]">
          <TriElementHeader
            columns={[
              <div class="text-right text-trades-sublabel">{"Price (USDT)"}</div>,
              <div class="text-right text-trades-sublabel">{"Quantity (BTC)"}</div>,
              <div class="text-right text-trades-sublabel">{"Time"}</div>,
            ]}
          />
        </div>
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute top-0 left-0 right-0 bottom-0 flex flex-col overflow-clip">
          <Index each={store.trades}>
            {(element, i) => (
              <TriElement
                class={`py-[3px] pr-[12px] text-right font-sanspro text-trades-item ${i % 2 ? "bg-gray-3" : ""}`}
                columns={[element().columns[0], element().columns[1], element().columns[2]]}
              />
            )}
          </Index>
        </div>
      </div>
    </div>
  );
}
