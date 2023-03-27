import { createEffect, Index, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import { TriElementComponent } from "./TriElement/TriElement";
import TriElement from "./TriElement/TriElement";
import TriElementHeader from "./TriElement/TriElementHeader";
import { Trade } from "~/types/trade";
import { PUBLIC_URL } from "~/types/mod";
import params from "~/utils/params";
import { Uuid } from "~/types/primitives/uuid";
import { z } from "zod";
import { truncateNumber } from "~/formatters/NumberFormatter";

export default function Trades() {
  const [storeState, setStoreState] = createStore<{ quote_asset_id: Uuid; base_asset_id: Uuid; capacity: number }>({
    quote_asset_id: "5864a1b9-4ae1-424f-bdb4-f644cb359463",
    base_asset_id: "7a99f052-d941-4dcc-aabd-6695c24deed5",
    capacity: 40,
  });
  const [storeComponent, setStoreComponent] = createStore<{ trades: TriElementComponent[] }>({ trades: [] });
  const [storeTrades, setStoreTrades] = createStore<{ trades: Trade[] }>({ trades: [] });

  onMount(async () => {
    const events = await new EventSource(
      `${PUBLIC_URL}/trades/sse?${params({
        quote_asset_id: storeState.quote_asset_id,
        base_asset_id: storeState.base_asset_id,
      })}`
    );
    events.onopen = async () => {
      const elements = await fetch(
        `${PUBLIC_URL}/trades?${params({
          quote_asset_id: storeState.quote_asset_id,
          base_asset_id: storeState.base_asset_id,
          limit: storeState.capacity,
          offset: 0,
        })}`
      )
        .then((r) => r.json())
        .then((r) => z.array(Trade).parse(r));
      setStoreTrades("trades", (prev) => [...elements, ...prev].slice(0, storeState.capacity));
    };
    events.onmessage = (msg) => {
      setStoreTrades("trades", (prev) => [Trade.parse(JSON.parse(msg.data)), ...prev].slice(0, storeState.capacity));
    };
  });

  createEffect(() => {
    const display = storeTrades.trades.map<TriElementComponent>((el) => ({
      column_0: (
        <span class={`${el.quote_asset_id == storeState.quote_asset_id && el.base_asset_id == storeState.base_asset_id ? "text-green" : "text-red"}`}>
          {truncateNumber(el.taker_base_volume / el.taker_quote_volume, 10)}
        </span>
      ),
      column_1: el.taker_base_volume.toString(),
      column_2: el.created_at.toLocaleTimeString(),
    }));
    setStoreComponent({ trades: display });
  });

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
          <Index each={storeComponent.trades}>
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
