import { Index, onCleanup, onMount, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { TriElementComponent } from "~/components/App/TriElement/TriElement";
import TriElement from "~/components/App/TriElement/TriElement";
import TriElementHeader from "~/components/App/TriElement/TriElementHeader";
import { PublicTrade } from "@packages/types/trade";
import params from "@packages/utils/params";
import { z } from "zod";
import { format } from "numerable";
import { formatTemplate } from "@packages/utils/precision";
import { api } from "~/root";
import { Asset } from "@packages/types/asset";
import { Market } from "~/components/providers/MarketProvider";
import { ev, finv, fmul } from "@packages/types/primitives/fraction";
import subscribeEvents from "@packages/utils/subscribeEvents";

export default function CreateTrades(
  market: Market,
  precision?: number,
  capacity?: number,
) {
  return () => (
    <Show
      when={
        market &&
        market.quote_asset &&
        market.base_asset &&
        precision &&
        capacity
      }
      fallback={<Trades />}
    >
      <Trades
        quote_asset={market.quote_asset}
        base_asset={market.base_asset}
        precision={precision}
        capacity={capacity}
      />
    </Show>
  );
}

export function Trades(props: {
  quote_asset?: Asset;
  base_asset?: Asset;
  precision?: number;
  capacity?: number;
}) {
  const [tradesState, setTradesState] = createStore<{
    trades: TriElementComponent[];
  }>({ trades: [] });

  let eventsource: EventSource | undefined;

  onMount(async () => {
    if (
      props.quote_asset &&
      props.base_asset &&
      props.precision &&
      props.capacity
    ) {
      const quote_asset = props.quote_asset;
      const base_asset = props.base_asset;
      const precision = props.precision;
      const capacity = props.capacity;

      const convertTrade = (trade: PublicTrade) => {
        return {
          column_0: (
            <span class={trade.direction == "buy" ? "text-green" : "text-red"}>
              {format(ev(trade.price), formatTemplate(precision))}
            </span>
          ),
          column_1: format(
            ev(fmul(trade.volume, finv(trade.price))),
            formatTemplate(precision),
          ),
          column_2: trade.time.toLocaleTimeString(),
        };
      };

      eventsource = await subscribeEvents(
        `${api}/public/trades`,
        params({
          quote_asset_id: quote_asset.id,
          base_asset_id: base_asset.id,
          limit: capacity,
          offset: 0,
        }),
        params({
          quote_asset_id: quote_asset.id,
          base_asset_id: base_asset.id,
        }),
        (data) => {
          setTradesState("trades", (prev) =>
            [
              ...prev,
              ...z.array(PublicTrade).parse(data).map(convertTrade),
            ].slice(0, props.capacity),
          );
        },
      );
    }
  });

  onCleanup(() => {
    eventsource?.close();
  });

  return (
    <div class="grid h-full grid-rows-[auto_36px_1fr]">
      <div class="row-start-1 row-end-2 p-3 font-sanspro text-orderbook-label font-semibold">
        Trades
      </div>
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
                class={`px-3 py-1 font-sanspro text-trades-item ${
                  i % 2 && "bg-gray-3"
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
