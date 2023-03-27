import { format } from "numerable";
import { createEffect, Index, JSX, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import { z } from "zod";
import { PriceLevel } from "~/api/public/mod";
import { PUBLIC_URL } from "~/types/mod";
import { Uuid } from "~/types/primitives/uuid";
import { Trade } from "~/types/trade";
import params from "~/utils/params";
import TriElementFill, { TriElementFillComponent } from "./TriElement/TriElementFill";
import TriElementHeader from "./TriElement/TriElementHeader";

export default function Orderbook() {
  const [storeState, setStoreState] = createStore<{ quote_asset_id: Uuid; base_asset_id: Uuid; number_pattern: string, capacity: number }>({
    quote_asset_id: "5864a1b9-4ae1-424f-bdb4-f644cb359463",
    base_asset_id: "7a99f052-d941-4dcc-aabd-6695c24deed5",
    number_pattern: "0,0.00",
    capacity: 10,
  });
  const [storeComponent, setStoreComponent] = createStore<{
    asks: TriElementFillComponent[],
    price: JSX.Element,
    bids: TriElementFillComponent[]
  }>({
    asks: [],
    price: "",
    bids: [],
  });

  const [storeOrderbook, setStoreOrderbook] = createStore<{
    asks: PriceLevel[],
    last_trade?: Trade,
    bids: PriceLevel[]
  }>({
    asks: [],
    last_trade: undefined,
    bids: []
  });

  onMount(async () => {
    const bids_events = await new EventSource(
      `${PUBLIC_URL}/depth/sse?${params({
        quote_asset_id: storeState.quote_asset_id,
        base_asset_id: storeState.base_asset_id,
        limit: storeState.capacity,
        precision: 2,
      })}`
    );
    bids_events.onopen = async () => {
      const elements = await fetch(
        `${PUBLIC_URL}/depth?${params({
          quote_asset_id: storeState.quote_asset_id,
          base_asset_id: storeState.base_asset_id,
          limit: storeState.capacity,
          precision: 2,
        })}`
      )
        .then((r) => r.json())
        .then((r) => z.array(PriceLevel).parse(r));
      setStoreOrderbook("bids", (prev) => prev.length > 0 ? prev : elements);
    };
    bids_events.onmessage = (msg) => {
      setStoreOrderbook("bids", z.array(PriceLevel).parse(JSON.parse(msg.data)));
    };

    const asks_events = await new EventSource(
      `${PUBLIC_URL}/depth/sse?${params({
        quote_asset_id: storeState.base_asset_id,
        base_asset_id: storeState.quote_asset_id,
        limit: storeState.capacity,
        precision: 2,
      })}`
    );
    asks_events.onopen = async () => {
      const elements = await fetch(
        `${PUBLIC_URL}/depth?${params({
          quote_asset_id: storeState.base_asset_id,
          base_asset_id: storeState.quote_asset_id,
          limit: storeState.capacity,
          precision: 2,
        })}`
      )
        .then((r) => r.json())
        .then((r) => z.array(PriceLevel).parse(r));
      setStoreOrderbook("asks", (prev) => prev.length > 0 ? prev : elements);
    };
    asks_events.onmessage = (msg) => {
      setStoreOrderbook("asks", z.array(PriceLevel).parse(JSON.parse(msg.data)));
    };

    const trades_events = await new EventSource(
      `${PUBLIC_URL}/trades/sse?${params({
        quote_asset_id: storeState.quote_asset_id,
        base_asset_id: storeState.base_asset_id,
      })}`
    );
    trades_events.onopen = async () => {
      const elements = await fetch(
        `${PUBLIC_URL}/trades?${params({
          quote_asset_id: storeState.quote_asset_id,
          base_asset_id: storeState.base_asset_id,
          limit: 1,
          offset: 0,
        })}`
      )
        .then((r) => r.json())
        .then((r) => z.array(Trade).parse(r)[0]);
      setStoreOrderbook("last_trade", (prev) => prev == undefined ? elements : prev);
    };
    trades_events.onmessage = (msg) => {
      setStoreOrderbook("last_trade", Trade.parse(JSON.parse(msg.data)));
    };

  });

  createEffect(() => {
    let total = 0;
    storeOrderbook.bids.forEach((el) => total += el.volume)
    let sum = 0
    const display = storeOrderbook.bids.map<TriElementFillComponent>((el) => {
      sum += el.volume;
      return {
        column_0: <span class="text-green">{format(el.price, storeState.number_pattern)}</span>,
        column_1: format(el.volume, storeState.number_pattern),
        column_2: format(sum, storeState.number_pattern),
        fill: sum / total,
        fill_class: "bg-green",
      }
    });
    setStoreComponent("bids", display);
  });

  createEffect(() => {
    let total = 0;
    storeOrderbook.asks.forEach((el) => total += el.volume)
    let sum = 0
    const display = storeOrderbook.asks.map<TriElementFillComponent>((el) => {
      sum += el.volume;
      return {
        column_0: <span class="text-red">{format(1 / el.price, storeState.number_pattern)}</span>,
        column_1: format(el.volume, storeState.number_pattern),
        column_2: format(sum, storeState.number_pattern),
        fill: sum / total,
        fill_class: "bg-red",
      }
    });
    setStoreComponent("asks", display);
  });

  createEffect(() => {
    const display = storeOrderbook.last_trade ?
      <span class={`${storeOrderbook.last_trade.quote_asset_id == storeState.quote_asset_id && storeOrderbook.last_trade.base_asset_id == storeState.base_asset_id ? "text-green" : "text-red"}`}>
        {format(storeOrderbook.last_trade.taker_base_volume / storeOrderbook.last_trade.taker_quote_volume, storeState.number_pattern)}
      </span>
      : undefined
    setStoreComponent("price", display);
  });

  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr_auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="p-4 font-sanspro text-orderbook-label font-semibold">Orderbook</div>
        <TriElementHeader
          class="py-[4px] px-[12px]"
          column_0={<div class="text-left text-orderbook-sublabel">{"Price (USDT)"}</div>}
          column_1={<div class="text-right text-orderbook-sublabel">{"Quantity (BTC)"}</div>}
          column_2={<div class="text-right text-orderbook-sublabel">{"Total (BTC)"}</div>}
        />
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute top-0 left-0 right-0 bottom-0 flex flex-col-reverse overflow-clip">
          <Index each={storeComponent.asks}>
            {(element) => (
              <TriElementFill
                class="py-[4px] px-[12px] font-sanspro text-orderbook-item"
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
      <div class="row-start-3 row-end-4 min-h-[50px]">
        <div class="flex flex-col px-[12px] justify-center h-full">
          <div class="text-orderbook-middle font-semibold ">{storeComponent.price}</div>
        </div>
      </div>
      <div class="relative row-start-4 row-end-5">
        <div class="absolute top-0 left-0 right-0 bottom-0 flex flex-col overflow-clip">
          <Index each={storeComponent.bids}>
            {(element) => (
              <TriElementFill
                class="py-[4px] px-[12px] font-sanspro text-orderbook-item"
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
