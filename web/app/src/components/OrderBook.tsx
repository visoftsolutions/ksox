import { format } from "numerable";
import { createEffect, Index, JSX, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import { joinPaths } from "solid-start/islands/server-router";
import { z } from "zod";
import { PriceLevel } from "~/api/public/mod";
import { Asset } from "~/types/asset";
import { Trade } from "~/types/trade";
import params from "~/utils/params";
import { formatTemplate } from "~/utils/precission";
import TriElementFill, { TriElementFillComponent } from "./TriElement/TriElementFill";
import TriElementHeader from "./TriElement/TriElementHeader";

export default function Orderbook() {
  const [stateState, setStoreState] = createStore<{ quote_asset: Asset; base_asset: Asset; precission: number; capacity: number }>({
    quote_asset: {
      id: "5864a1b9-4ae1-424f-bdb4-f644cb359463",
      created_at: new Date(),
      name: "bitcoin",
      symbol: "USDC",
      maker_fee: {
        numerator: 10,
        denominator: 10,
      },
      taker_fee: {
        numerator: 10,
        denominator: 10,
      },
    },
    base_asset: {
      id: "7a99f052-d941-4dcc-aabd-6695c24deed5",
      created_at: new Date(),
      name: "ethereum",
      symbol: "ETH",
      maker_fee: {
        numerator: 10,
        denominator: 10,
      },
      taker_fee: {
        numerator: 10,
        denominator: 10,
      },
    },
    precission: 3,
    capacity: 10,
  });
  const [stateComponent, setStoreComponent] = createStore<{
    asks: TriElementFillComponent[];
    price: JSX.Element;
    bids: TriElementFillComponent[];
  }>({
    asks: [],
    price: undefined,
    bids: [],
  });

  const [stateOrderbook, setStoreOrderbook] = createStore<{
    asks: PriceLevel[];
    last_trade?: Trade;
    bids: PriceLevel[];
  }>({
    asks: [],
    last_trade: undefined,
    bids: [],
  });

  onMount(async () => {
    const BASE_URL = location.pathname;
    const API_URL = joinPaths(BASE_URL, "/api");
    const PUBLIC_URL = joinPaths(API_URL, "/public");

    const bids_events = new EventSource(
      `${PUBLIC_URL}/depth/sse?${params({
        quote_asset_id: stateState.quote_asset.id,
        base_asset_id: stateState.base_asset.id,
        limit: stateState.capacity,
        precision: 2,
      })}`
    );
    bids_events.onopen = async () => {
      const elements = await fetch(
        `${PUBLIC_URL}/depth?${params({
          quote_asset_id: stateState.quote_asset.id,
          base_asset_id: stateState.base_asset.id,
          limit: stateState.capacity,
          precision: 2,
        })}`
      )
        .then((r) => r.json())
        .then((r) => z.array(PriceLevel).parse(r));
      setStoreOrderbook("bids", (prev) => (prev.length > 0 ? prev : elements.reverse()));
    };
    bids_events.onmessage = (msg) => {
      setStoreOrderbook("bids", z.array(PriceLevel).parse(JSON.parse(msg.data)).reverse());
    };

    const asks_events = new EventSource(
      `${PUBLIC_URL}/depth/sse?${params({
        quote_asset_id: stateState.base_asset.id,
        base_asset_id: stateState.quote_asset.id,
        limit: stateState.capacity,
        precision: 2,
      })}`
    );
    asks_events.onopen = async () => {
      const elements = await fetch(
        `${PUBLIC_URL}/depth?${params({
          quote_asset_id: stateState.base_asset.id,
          base_asset_id: stateState.quote_asset.id,
          limit: stateState.capacity,
          precision: 2,
        })}`
      )
        .then((r) => r.json())
        .then((r) => z.array(PriceLevel).parse(r));
      setStoreOrderbook("asks", (prev) => (prev.length > 0 ? prev : elements.reverse()));
    };
    asks_events.onmessage = (msg) => {
      setStoreOrderbook("asks", z.array(PriceLevel).parse(JSON.parse(msg.data)).reverse());
    };

    const trades_events = new EventSource(
      `${PUBLIC_URL}/trades/sse?${params({
        quote_asset_id: stateState.quote_asset.id,
        base_asset_id: stateState.base_asset.id,
      })}`
    );
    trades_events.onopen = async () => {
      const elements = await fetch(
        `${PUBLIC_URL}/trades?${params({
          quote_asset_id: stateState.quote_asset.id,
          base_asset_id: stateState.base_asset.id,
          limit: 1,
          offset: 0,
        })}`
      )
        .then((r) => r.json())
        .then((r) => z.array(Trade).parse(r)[0]);
      setStoreOrderbook("last_trade", (prev) => (prev == undefined ? elements : prev));
    };
    trades_events.onmessage = (msg) => {
      setStoreOrderbook("last_trade", Trade.parse(JSON.parse(msg.data)));
    };
  });

  createEffect(() => {
    let total = 0;
    stateOrderbook.bids.forEach((el) => (total += el.volume));
    let sum = 0;
    const display = stateOrderbook.bids.map<TriElementFillComponent>((el) => {
      sum += el.volume;
      return {
        column_0: <span class="text-green">{format(el.price, formatTemplate(stateState.precission))}</span>,
        column_1: format(el.volume, formatTemplate(stateState.precission)),
        column_2: format(sum, formatTemplate(stateState.precission)),
        fill: sum / total,
        fill_class: "bg-green",
      };
    });
    setStoreComponent("bids", display);
  });

  createEffect(() => {
    let total = 0;
    stateOrderbook.asks.forEach((el) => (total += el.volume));
    let sum = 0;
    const display = stateOrderbook.asks.map<TriElementFillComponent>((el) => {
      sum += el.volume;
      return {
        column_0: <span class="text-red">{format(1 / el.price, formatTemplate(stateState.precission))}</span>,
        column_1: format(el.volume, formatTemplate(stateState.precission)),
        column_2: format(sum, formatTemplate(stateState.precission)),
        fill: sum / total,
        fill_class: "bg-red",
      };
    });
    setStoreComponent("asks", display);
  });

  createEffect(() => {
    const display = stateOrderbook.last_trade ? (
      <span
        class={`${
          stateOrderbook.last_trade.quote_asset_id == stateState.quote_asset.id && stateOrderbook.last_trade.base_asset_id == stateState.base_asset.id
            ? "text-green"
            : "text-red"
        }`}
      >
        {format(stateOrderbook.last_trade.taker_base_volume / stateOrderbook.last_trade.taker_quote_volume, formatTemplate(stateState.precission))}
      </span>
    ) : undefined;
    setStoreComponent("price", display);
  });

  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr_auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="p-4 font-sanspro text-orderbook-label font-semibold">Orderbook</div>
        <TriElementHeader
          class="py-[4px] px-[12px]"
          column_0={<div class="text-left text-orderbook-sublabel">{`Price (${stateState.quote_asset.symbol})`}</div>}
          column_1={<div class="text-right text-orderbook-sublabel">{`Quantity (${stateState.base_asset.symbol})`}</div>}
          column_2={<div class="text-right text-orderbook-sublabel">{`Total (${stateState.quote_asset.symbol})`}</div>}
        />
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute top-0 left-0 right-0 bottom-0 flex flex-col-reverse overflow-clip">
          <Index each={stateComponent.asks}>
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
        <div class="flex h-full flex-col justify-center px-[12px]">
          <div class="text-orderbook-middle font-semibold ">{stateComponent.price}</div>
        </div>
      </div>
      <div class="relative row-start-4 row-end-5">
        <div class="absolute top-0 left-0 right-0 bottom-0 flex flex-col overflow-clip">
          <Index each={stateComponent.bids}>
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
