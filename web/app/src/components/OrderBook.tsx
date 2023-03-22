import { Index, JSX } from "solid-js";
import { createStore } from "solid-js/store";
import { init } from "~/memos/Orderbook";
import TriElementFill, { TriElementFillComponent } from "./TriElement/TriElementFill";
import TriElementHeader from "./TriElement/TriElementHeader";

export interface OrderbookComponent {
  asks: TriElementFillComponent[];
  price: JSX.Element;
  bids: TriElementFillComponent[];
}

export const [store, setStore] = createStore<OrderbookComponent>({
  asks: [],
  price: "",
  bids: [],
});

init();

export default function Orderbook() {
  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr_auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="p-4 font-sanspro text-orderbook-label font-semibold">Orderbook</div>
        <div class="pr-[12px]">
          <TriElementHeader
            column_0={<div class="text-right text-orderbook-sublabel">{"Price (USDT)"}</div>}
            column_1={<div class="text-right text-orderbook-sublabel">{"Quantity (BTC)"}</div>}
            column_2={<div class="text-right text-orderbook-sublabel">{"Total (BTC)"}</div>}
          />
        </div>
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute top-0 left-0 right-0 bottom-0 flex flex-col-reverse overflow-clip">
          <Index each={store.asks}>
            {(element) => (
              <TriElementFill
                class="py-[3px] pr-[12px] text-right font-sanspro text-orderbook-item"
                column_0={element().column_0}
                column_1={element().column_1}
                column_2={element().column_2}
                fill={element().fill}
                fill_class={element().fill_class}
              />
            )}
          </Index>
        </div>
      </div>
      <div class="row-start-3 row-end-4">
        <div class="p-4 text-orderbook-middle font-semibold">{store.price}</div>
      </div>
      <div class="relative row-start-4 row-end-5">
        <div class="absolute top-0 left-0 right-0 bottom-0 flex flex-col overflow-clip">
          <Index each={store.bids}>
            {(element) => (
              <TriElementFill
                class="py-[3px] text-right font-sanspro text-orderbook-item"
                column_0={element().column_0}
                column_1={element().column_1}
                column_2={element().column_2}
                fill={element().fill}
                fill_class={element().fill_class}
              />
            )}
          </Index>
        </div>
      </div>
    </div>
  );
}
