import { ethers } from "ethers";
import { format } from "numerable";
import { Index, Match, onCleanup, onMount, Show, Switch } from "solid-js";
import { createStore } from "solid-js/store";
import { z } from "zod";
import { api } from "~/root";
import { Asset } from "~/types/asset";
import { Volume } from "~/types/primitives/volume";
import { Trade } from "~/types/trade";
import params from "~/utils/params";
import { formatTemplate } from "~/utils/precision";
import { Market } from "~/utils/providers/MarketProvider";
import TriElementFill, { TriElementFillComponent } from "./TriElement/TriElementFill";
import TriElementHeader from "./TriElement/TriElementHeader";

export const PriceLevel = z.object({
  price: z.number(),
  volume: Volume,
});
export type PriceLevel = z.infer<typeof PriceLevel>;

export default function CreateOrderBook(market: Market, precision?: number, capacity?: number) {
  return () => (
    <Switch fallback={<OrderBook />}>
      <Match when={market && market.quote_asset && market.base_asset && precision && capacity}>
        <OrderBook quote_asset={market.quote_asset} base_asset={market.base_asset} precision={precision} capacity={capacity} />
      </Match>
    </Switch>
  );
}

export function OrderBook(props: { quote_asset?: Asset; base_asset?: Asset; precision?: number; capacity?: number }) {
  const [orderBookState, setOrderBookState] = createStore<{
    buys: TriElementFillComponent[];
    last_price: string | null;
    sells: TriElementFillComponent[];
  }>({
    buys: [],
    last_price: null,
    sells: [],
  });

  let buys_events: EventSource | null = null;
  let sells_events: EventSource | null = null;
  let trades_events: EventSource | null = null;

  onMount(() => {
    if (props.quote_asset && props.base_asset && props.precision && props.capacity) {
      const quote_asset = props.quote_asset;
      const base_asset = props.base_asset;
      const precision = props.precision;
      const capacity = props.capacity;

      buys_events = new EventSource(
        `${api}/public/depth/sse?${params({
          quote_asset_id: quote_asset.id,
          base_asset_id: base_asset.id,
          limit: capacity,
          precision: precision,
        })}`
      );
      buys_events.onopen = async () =>
        await fetch(
          `${api}/public/depth?${params({
            quote_asset_id: quote_asset.id,
            base_asset_id: base_asset.id,
            limit: capacity,
            precision: precision,
          })}`
        )
          .then((r) => r.json())
          .then((r) => z.array(PriceLevel).parse(r))
          .then((r) => {
            let total = 0,
              sum = 0;
            r.forEach((value) => {
              total += Number(ethers.utils.formatEther(value.volume));
            });
            console.log(r);
            setOrderBookState(
              "buys",
              r.map<TriElementFillComponent>((el) => {
                const volume = Number(ethers.utils.formatEther(el.volume));
                sum += volume;
                return {
                  column_0: <span class="text-green">{format(el.price, formatTemplate(precision))}</span>,
                  column_1: format(volume, formatTemplate(precision)),
                  column_2: format(sum, formatTemplate(precision)),
                  fill: sum / total,
                  fill_class: "bg-green",
                };
              })
            );
          });
      buys_events.onmessage = (ev) => {
        let total = 0,
          sum = 0;
        const buys = z.array(PriceLevel).parse(JSON.parse(ev.data));
        buys.forEach((value) => {
          total += Number(ethers.utils.formatEther(value.volume));
        });
        setOrderBookState(
          "buys",
          buys.map<TriElementFillComponent>((el) => {
            const volume = Number(ethers.utils.formatEther(el.volume));
            sum += volume;
            return {
              column_0: <span class="text-green">{format(el.price, formatTemplate(precision))}</span>,
              column_1: format(volume, formatTemplate(precision)),
              column_2: format(sum, formatTemplate(precision)),
              fill: sum / total,
              fill_class: "bg-green",
            };
          })
        );
      };

      sells_events = new EventSource(
        `${api}/public/depth/sse?${params({
          quote_asset_id: base_asset.id,
          base_asset_id: quote_asset.id,
          limit: capacity,
          precision: precision,
        })}`
      );
      sells_events.onopen = async () =>
        await fetch(
          `${api}/public/depth?${params({
            quote_asset_id: base_asset.id,
            base_asset_id: quote_asset.id,
            limit: capacity,
            precision: precision,
          })}`
        )
          .then((r) => r.json())
          .then((r) => z.array(PriceLevel).parse(r))
          .then((r) => {
            let total = 0,
              sum = 0;
            r.forEach((value) => {
              total += Number(ethers.utils.formatEther(value.volume));
            });
            setOrderBookState(
              "sells",
              r.map<TriElementFillComponent>((el) => {
                const volume = Number(ethers.utils.formatEther(el.volume));
                sum += volume;
                return {
                  column_0: <span class="text-red">{format(el.price, formatTemplate(precision))}</span>,
                  column_1: format(volume, formatTemplate(precision)),
                  column_2: format(sum, formatTemplate(precision)),
                  fill: sum / total,
                  fill_class: "bg-red",
                };
              })
            );
          });
      sells_events.onmessage = (ev) => {
        let total = 0,
          sum = 0;
        const sells = z.array(PriceLevel).parse(JSON.parse(ev.data));
        sells.forEach((value) => {
          total += Number(ethers.utils.formatEther(value.volume));
        });
        setOrderBookState(
          "sells",
          sells.map<TriElementFillComponent>((el) => {
            const volume = Number(ethers.utils.formatEther(el.volume));
            sum += volume;
            return {
              column_0: <span class="text-red">{format(el.price, formatTemplate(precision))}</span>,
              column_1: format(volume, formatTemplate(precision)),
              column_2: format(sum, formatTemplate(precision)),
              fill: sum / total,
              fill_class: "bg-red",
            };
          })
        );
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
          .then((r) => z.array(Trade).parse(r)[0])
          .then((r) => {
            if (r) {
              const taker_quote_volume = Number(ethers.utils.formatEther(r.taker_quote_volume));
              const taker_base_volume = Number(ethers.utils.formatEther(r.taker_base_volume));
              if (r.quote_asset_id == quote_asset.id && r.base_asset_id == base_asset.id) {
                setOrderBookState("last_price", format(taker_base_volume / taker_quote_volume, formatTemplate(precision)));
              } else if (r.quote_asset_id == base_asset.id && r.base_asset_id == quote_asset.id) {
                setOrderBookState("last_price", format(taker_quote_volume / taker_base_volume, formatTemplate(precision)));
              }
            }
          });
      trades_events.onmessage = (ev) => {
        const last_trade = Trade.parse(JSON.parse(ev.data));
        const taker_quote_volume = Number(ethers.utils.formatEther(last_trade.taker_quote_volume));
        const taker_base_volume = Number(ethers.utils.formatEther(last_trade.taker_base_volume));
        if (last_trade.quote_asset_id == quote_asset.id && last_trade.base_asset_id == base_asset.id) {
          setOrderBookState("last_price", format(taker_base_volume / taker_quote_volume, formatTemplate(precision)));
        } else if (last_trade.quote_asset_id == base_asset.id && last_trade.base_asset_id == quote_asset.id) {
          setOrderBookState("last_price", format(taker_quote_volume / taker_base_volume, formatTemplate(precision)));
        }
      };
    }
  });

  onCleanup(() => {
    buys_events?.close();
    sells_events?.close();
    trades_events?.close();
  });

  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr_auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="p-4 font-sanspro text-orderbook-label font-semibold">Orderbook</div>
        <TriElementHeader
          class="px-[12px] py-[4px]"
          column_0={<div class="text-left text-orderbook-sublabel">{`Price (${props.quote_asset ? props.quote_asset.symbol : "---"})`}</div>}
          column_1={<div class="text-right text-orderbook-sublabel">{`Quantity (${props.base_asset ? props.base_asset.symbol : "---"})`}</div>}
          column_2={<div class="text-right text-orderbook-sublabel">{`Total (${props.quote_asset ? props.quote_asset.symbol : "---"})`}</div>}
        />
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col-reverse overflow-clip">
          <Index each={orderBookState.sells}>
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
      <div class="row-start-3 row-end-4 min-h-[50px]">
        <div class="flex h-full flex-col justify-center px-[12px]">
          <div class="text-orderbook-middle font-semibold ">{orderBookState.last_price}</div>
        </div>
      </div>
      <div class="relative row-start-4 row-end-5">
        <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col overflow-clip">
          <Index each={orderBookState.buys}>
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
    </div>
  );
}
