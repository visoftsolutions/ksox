import { format } from "numerable";
import { Index, JSXElement, onCleanup, onMount, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { z } from "zod";
import { api } from "~/root";
import { Asset } from "~/types/asset";
import params from "~/utils/params";
import { formatTemplate } from "~/utils/precision";
import { Market } from "~/utils/providers/MarketProvider";
import TriElementFill, { TriElementFillComponent } from "./TriElement/TriElementFill";
import TriElementHeader from "./TriElement/TriElementHeader";
import { ev } from "~/types/primitives/fraction";
import { PriceLevel } from "~/types/pricelevel";
import { PublicTrade } from "~/types/trade";

export const DepthResponse = z.object({
  sells: z.array(PriceLevel),
  buys: z.array(PriceLevel),
});

export type DepthResponse = z.infer<typeof DepthResponse>;

export default function CreateOrderBook(market: Market, precision?: number, capacity?: number) {
  return () => (
    <Show when={market && market.quote_asset && market.base_asset && precision && capacity} fallback={<OrderBook />}>
      <OrderBook quote_asset={market.quote_asset} base_asset={market.base_asset} precision={precision} capacity={capacity} />
    </Show>
  );
}

export function OrderBook(props: { quote_asset?: Asset; base_asset?: Asset; precision?: number; capacity?: number }) {
  const [orderBook, setOrderBook] = createStore<{
    buys: TriElementFillComponent[];
    price: JSXElement;
    sells: TriElementFillComponent[];
  }>({
    buys: [],
    price: null,
    sells: [],
  });

  let depth_events: EventSource | null = null;
  let trades_events: EventSource | null = null;

  onMount(() => {
    if (props.quote_asset && props.base_asset && props.precision && props.capacity) {
      const quote_asset = props.quote_asset;
      const base_asset = props.base_asset;
      const precision = props.precision;
      const capacity = props.capacity;

      const convertTrade = (trade: PublicTrade) => {
        return <span class={trade.direction == "buy" ? "text-green" : "text-red"}>{format(ev(trade.price), formatTemplate(precision))}</span>;
      };

      const convertBuys = (levels: PriceLevel[]) => {
        let total = 0,
          sum = 0;
        levels.forEach((value) => (total += ev(value.volume)));
        return levels.map<TriElementFillComponent>((el) => {
          const price = ev(el.price);
          const volume = ev(el.volume);
          sum += volume;
          return {
            column_0: <span class="text-green">{format(price, formatTemplate(precision))}</span>,
            column_1: format(volume / price, formatTemplate(precision)),
            column_2: format(sum, formatTemplate(precision)),
            fill: sum / total,
            fill_class: "bg-green",
          };
        });
      };

      const convertSells = (levels: PriceLevel[]) => {
        let total = 0,
          sum = 0;
        levels.forEach((value) => (total += ev(value.volume)));
        return levels.map<TriElementFillComponent>((el) => {
          const price = ev(el.price);
          const volume = ev(el.volume);
          sum += volume;
          return {
            column_0: <span class="text-red">{format(price, formatTemplate(precision))}</span>,
            column_1: format(volume / price, formatTemplate(precision)),
            column_2: format(sum, formatTemplate(precision)),
            fill: sum / total,
            fill_class: "bg-red",
          };
        });
      };

      depth_events = new EventSource(
        `${api}/public/depth/sse?${params({
          quote_asset_id: quote_asset.id,
          base_asset_id: base_asset.id,
          limit: capacity,
          precision: 100,
        })}`
      );
      depth_events.onopen = async () =>
        await fetch(
          `${api}/public/depth?${params({
            quote_asset_id: quote_asset.id,
            base_asset_id: base_asset.id,
            limit: capacity,
            precision: 100,
          })}`
        )
          .then((r) => r.json())
          .then((r) => DepthResponse.parse(r))
          .then((r) => {
            const b = convertBuys(r.buys);
            const s = convertSells(r.sells);
            setOrderBook("buys", b);
            setOrderBook("sells", s);
          });
      depth_events.onmessage = (ev) => {
        const r = DepthResponse.parse(JSON.parse(ev.data));
        const b = convertBuys(r.buys);
        const s = convertSells(r.sells);
        setOrderBook("buys", b);
        setOrderBook("sells", s);
      };

      trades_events = new EventSource(
        `${api}/public/trades/sse?${params({
          quote_asset_id: quote_asset.id,
          base_asset_id: base_asset.id,
        })}`
      );
      trades_events.onopen = async () =>
        await fetch(
          `${api}/public/trades?${params({
            quote_asset_id: quote_asset.id,
            base_asset_id: base_asset.id,
            limit: 1,
            offset: 0,
          })}`
        )
          .then((r) => r.json())
          .then((r) => z.array(PublicTrade).parse(r))
          .then((r) => {
            if (r && r.length > 0) setOrderBook("price", convertTrade(r[0]));
          });

      trades_events.onmessage = (ev) => {
        const trades = PublicTrade.array().parse(JSON.parse(ev.data));
        if (trades.length > 0) {
          setOrderBook("price", convertTrade(trades[0]));
        }
      };
    }
  });

  onCleanup(() => {
    depth_events?.close();
    trades_events?.close();
  });

  return (
    <div class="grid h-full grid-rows-[auto_36px_1fr_40px_1fr]">
      <div class="row-start-1 row-end-2 p-3 font-sanspro text-orderbook-label font-semibold">Orderbook</div>
      <TriElementHeader
        class="row-start-2 row-end-3 grid items-center justify-center px-3"
        column_0={`Price (${props.quote_asset?.symbol ?? "---"})`}
        column_1={`Quantity (${props.base_asset?.symbol ?? "---"})`}
        column_2={`Total (${props.quote_asset?.symbol ?? "---"})`}
      />
      <div class="relative row-start-3 row-end-4">
        <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col-reverse overflow-clip">
          <Index each={orderBook.sells}>
            {(element) => (
              <TriElementFill
                class="px-[12px] py-[4px] font-sanspro text-orderbook-item"
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
      <div class="row-start-4 row-end-5">
        <div class="flex h-full flex-col justify-center px-3">
          <div class="text-orderbookMiddle font-bold ">{orderBook.price}</div>
        </div>
      </div>
      <div class="relative row-start-5 row-end-6">
        <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col overflow-clip">
          <Index each={orderBook.buys}>
            {(element) => (
              <TriElementFill
                class="mt-[1px] px-3 py-1 font-sanspro text-orderbook-item"
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
