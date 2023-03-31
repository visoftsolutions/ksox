import { Index, Match, onCleanup, onMount, Switch } from "solid-js";
import { createStore } from "solid-js/store";
import { TriElementComponent } from "~/components/TriElement/TriElement";
import TriElement from "~/components/TriElement/TriElement";
import TriElementHeader from "~/components/TriElement/TriElementHeader";
import { Trade } from "~/types/trade";
import params from "~/utils/params";
import { z } from "zod";
import { format } from "numerable";
import { formatTemplate } from "~/utils/precision";
import { api } from "~/root";
import { fromWei } from "~/converters/wei";
import { Asset } from "~/types/asset";
import { Market } from "~/utils/providers/MarketProvider";

export default function CreateTrades(market: Market, precision?: number, capacity?: number) {
  return () => (
    <Switch fallback={<Trades />}>
      <Match when={market && market.quote_asset && market.base_asset && precision && capacity}>
        <Trades quote_asset={market.quote_asset} base_asset={market.base_asset} precision={precision} capacity={capacity} />
      </Match>
    </Switch>
  );
}

export function Trades(props: { quote_asset?: Asset; base_asset?: Asset; precision?: number; capacity?: number }) {
  const [tradesState, setTradesState] = createStore<{ trades: TriElementComponent[] }>({ trades: [] });

  let events: EventSource | null = null;

  onMount(() => {
    if (props.quote_asset && props.base_asset && props.precision && props.capacity) {
      const quote_asset = props.quote_asset;
      const base_asset = props.base_asset;
      const precision = props.precision;
      const capacity = props.capacity;

      events = new EventSource(
        `${api}/public/trades/sse?${params({
          quote_asset_id: quote_asset.id,
          base_asset_id: base_asset.id,
        })}`
      );
      events.onopen = async () =>
        await fetch(
          `${api}/public/trades?${params({
            quote_asset_id: quote_asset.id,
            base_asset_id: base_asset.id,
            limit: capacity,
            offset: 0,
          })}`
        )
          .then((r) => r.json())
          .then((r) => z.array(Trade).parse(r))
          .then((r) => {
            return r.map<TriElementComponent>((el) => {
              const taker_quote_volume = fromWei(el.taker_quote_volume);
              const taker_base_volume = fromWei(el.taker_base_volume);
              const price =
                el.quote_asset_id == quote_asset.id && el.base_asset_id == base_asset.id
                  ? taker_base_volume / taker_quote_volume
                  : taker_quote_volume / taker_base_volume;
              return {
                column_0: (
                  <span class={`${el.quote_asset_id == quote_asset.id && el.base_asset_id == base_asset.id ? "text-green" : "text-red"}`}>
                    {format(price, formatTemplate(precision))}
                  </span>
                ),
                column_1: format(taker_base_volume, formatTemplate(precision)),
                column_2: el.created_at.toLocaleTimeString(),
              };
            });
          })
          .then((r) => setTradesState("trades", r.slice(0, props.capacity)));
      events.onmessage = (ev) => {
        const last_trade = Trade.parse(JSON.parse(ev.data));
        const taker_quote_volume = fromWei(last_trade.taker_quote_volume);
        const taker_base_volume = fromWei(last_trade.taker_base_volume);
        const price =
          last_trade.quote_asset_id == quote_asset.id && last_trade.base_asset_id == base_asset.id
            ? taker_base_volume / taker_quote_volume
            : taker_quote_volume / taker_base_volume;
        const trade = {
          column_0: (
            <span class={`${last_trade.quote_asset_id == quote_asset.id && last_trade.base_asset_id == base_asset.id ? "text-green" : "text-red"}`}>
              {format(price, formatTemplate(precision))}
            </span>
          ),
          column_1: format(taker_base_volume, formatTemplate(precision)),
          column_2: last_trade.created_at.toLocaleTimeString(),
        };
        setTradesState("trades", (prev) => [trade, ...prev].slice(0, props.capacity));
      };
    }
  });

  onCleanup(() => {
    events?.close();
  });

  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="p-4 font-sanspro text-trades-label font-semibold">Trades</div>
        <TriElementHeader
          class="px-[12px] py-[4px]"
          column_0={<div class="text-left text-trades-sublabel">{`Price (${props.quote_asset ? props.quote_asset.symbol : "---"})`}</div>}
          column_1={<div class="text-right text-trades-sublabel">{`Quantity (${props.base_asset ? props.base_asset.symbol : "---"})`}</div>}
          column_2={<div class="text-right text-trades-sublabel">{"Time"}</div>}
        />
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col overflow-clip">
          <Index each={tradesState.trades}>
            {(element, i) => (
              <TriElement
                class={`px-[12px] py-[4px] font-sanspro text-trades-item ${i % 2 ? "bg-gray-3" : ""}`}
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
