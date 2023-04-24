import { Index, onCleanup, onMount, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { TriElementComponent } from "~/components/App/TriElement/TriElement";
import TriElement from "~/components/App/TriElement/TriElement";
import TriElementHeader from "~/components/App/TriElement/TriElementHeader";
import { PublicTrade } from "~/types/trade";
import params from "~/utils/params";
import { z } from "zod";
import { format } from "numerable";
import { formatTemplate } from "~/utils/precision";
import { api } from "~/root";
import { Asset } from "~/types/asset";
import { Market } from "~/utils/providers/MarketProvider";
import { ev } from "~/types/primitives/fraction";

export default function CreateTrades(market: Market, precision?: number, capacity?: number) {
  return () => (
    <Show when={market && market.quote_asset && market.base_asset && precision && capacity} fallback={<Trades />}>
      <Trades quote_asset={market.quote_asset} base_asset={market.base_asset} precision={precision} capacity={capacity} />
    </Show>
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

      const convertTrade = (trade: PublicTrade) => {
        return {
          column_0: <span class={trade.direction == "buy" ? "text-green" : "text-red"}>{format(ev(trade.price), formatTemplate(precision))}</span>,
          column_1: format(ev(trade.volume), formatTemplate(precision)),
          column_2: trade.time.toLocaleTimeString(),
        };
      };

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
          .then((r) => z.array(PublicTrade).parse(r))
          .then((r) => r.map<TriElementComponent | undefined>((e) => convertTrade(e)).filter((e): e is TriElementComponent => !!e))
          .then((r) => setTradesState("trades", (prev) => [...prev, ...r].slice(0, props.capacity)));
      events.onmessage = (ev) => {
        const trades = PublicTrade.array().parse(JSON.parse(ev.data)).map(convertTrade);
        if (trades) {
          setTradesState("trades", (prev) => [...trades, ...prev].slice(0, props.capacity));
        }
      };
    }
  });

  onCleanup(() => {
    events?.close();
  });

  return (
    <div class="grid h-full grid-rows-[auto_36px_1fr]">
      <div class="row-start-1 row-end-2 p-3 font-sanspro text-orderbook-label font-semibold">Trades</div>
      <TriElementHeader
        class="row-start-2 row-end-3 px-3"
        column_0={`Price (${props.quote_asset?.symbol ?? "---"})`}
        column_1={`Quantity (${props.base_asset?.symbol ?? "---"})`}
        column_2={"Time"}
      />
      <div class="relative row-start-3 row-end-4">
        <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col overflow-clip">
          <Index each={tradesState.trades}>
            {(element, i) => (
              <TriElement
                class={`px-3 py-1 font-sanspro text-trades-item ${i % 2 && "bg-gray-3"}`}
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
