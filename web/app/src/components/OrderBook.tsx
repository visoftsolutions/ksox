import { Index, JSX } from "solid-js";
import { createStore } from "solid-js/store";
import TriElementFill, { TriElementFillDisplay } from "./TriElement/TriElementFill";
import TriElementHeader from "./TriElement/TriElementHeader";

export interface OrderbookDisplay {
  asks: TriElementFillDisplay[];
  price: JSX.Element;
  bids: TriElementFillDisplay[];
}

const [store, setStore] = createStore<OrderbookDisplay>({
  asks: [],
  price: "",
  bids: [],
});

setStore({
  asks: [
    {
      columns: [<span class="text-green">{"22,342.12"}</span>, "0.002234", "0.002234"],
      fill: 0.2,
      fill_class: "bg-green",
    },
    {
      columns: [<span class="text-green">{"22,342.12"}</span>, "0.002234", "0.002234"],
      fill: 0.2,
      fill_class: "bg-green",
    },
  ],
  price: <span class="text-green">{"22,342.12"}</span>,
  bids: [
    {
      columns: [<span class="text-red">{"22,342.12"}</span>, "0.002234", "0.002234"],
      fill: 0.2,
      fill_class: "bg-red",
    },
    {
      columns: [<span class="text-red">{"22,342.12"}</span>, "0.002234", "0.002234"],
      fill: 0.5,
      fill_class: "bg-red",
    },
  ],
});

export default function Orderbook() {
  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr_auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="p-4 font-sanspro text-orderbook-label font-semibold">Orderbook</div>
        <div class="pr-[12px]">
          <TriElementHeader
            columns={[
              <div class="text-right text-orderbook-sublabel">{"Price (USDT)"}</div>,
              <div class="text-right text-orderbook-sublabel">{"Quantity (BTC)"}</div>,
              <div class="text-right text-orderbook-sublabel">{"Total (BTC)"}</div>,
            ]}
          />
        </div>
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute top-0 left-0 right-0 bottom-0 flex flex-col-reverse overflow-clip">
          <Index each={store.asks}>
            {(element) => (
              <TriElementFill
                class="py-[3px] pr-[12px] text-right font-sanspro text-orderbook-item"
                columns={[element().columns[0], element().columns[1], element().columns[2]]}
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
                columns={[element().columns[0], element().columns[1], element().columns[2]]}
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
