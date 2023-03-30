import { format } from "numerable";
import { Index, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import { z } from "zod";
import { PriceLevel } from "~/api/public/mod";
import { api } from "~/root";
import { Asset } from "~/types/asset";
import { Trade } from "~/types/trade";
import params from "~/utils/params";
import { formatTemplate } from "~/utils/precision";
import TriElementFill, { TriElementFillComponent } from "./TriElement/TriElementFill";
import TriElementHeader from "./TriElement/TriElementHeader";

export default function OrderBook(props: { quoteAsset?: Asset; baseAsset?: Asset; precission?: number; capacity?: number }) {
  const [orderBookState, setOrderBookState] = createStore<{
    buys: TriElementFillComponent[];
    lastPrice: string | null;
    sells: TriElementFillComponent[];
  }>({
    buys: [],
    lastPrice: null,
    sells: [],
  });

  onMount(async () => {
    if (props.quoteAsset && props.baseAsset && props.precission && props.capacity) {
      const quoteAsset = props.quoteAsset;
      const baseAsset = props.baseAsset;
      const precission = props.precission;
      const capacity = props.capacity;

      const buys_events = new EventSource(
        `${api}/public/depth/sse?${params({
          quoteAssetId: quoteAsset.id,
          baseAssetId: baseAsset.id,
          limit: capacity,
          precision: precission,
        })}`
      );
      buys_events.onopen = async () =>
        fetch(
          `${api}/public/depth?${params({
            quoteAssetId: quoteAsset.id,
            baseAssetId: baseAsset.id,
            limit: capacity,
            precision: precission,
          })}`
        )
          .then((r) => r.json())
          .then((r) => z.array(PriceLevel).parse(r))
          .then((r) => {
            let total = 0,
              sum = 0;
            r.forEach((value) => {
              total += value.volume;
            });
            setOrderBookState(
              "buys",
              r.map<TriElementFillComponent>((el) => {
                sum += el.volume;
                return {
                  column0: <span class="text-green">{format(el.price, formatTemplate(precission))}</span>,
                  column1: format(el.volume, formatTemplate(precission)),
                  column2: format(sum, formatTemplate(precission)),
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
          total += value.volume;
        });
        setOrderBookState(
          "buys",
          buys.map<TriElementFillComponent>((el) => {
            sum += el.volume;
            return {
              column0: <span class="text-green">{format(el.price, formatTemplate(precission))}</span>,
              column1: format(el.volume, formatTemplate(precission)),
              column2: format(sum, formatTemplate(precission)),
              fill: sum / total,
              fill_class: "bg-green",
            };
          })
        );
      };

      const sells_events = new EventSource(
        `${api}/public/depth/sse?${params({
          quoteAssetId: baseAsset.id,
          baseAssetId: quoteAsset.id,
          limit: capacity,
          precision: precission,
        })}`
      );
      sells_events.onopen = async () =>
        fetch(
          `${api}/public/depth?${params({
            quoteAssetId: baseAsset.id,
            baseAssetId: quoteAsset.id,
            limit: capacity,
            precision: precission,
          })}`
        )
          .then((r) => r.json())
          .then((r) => z.array(PriceLevel).parse(r).reverse())
          .then((r) => {
            let total = 0,
              sum = 0;
            r.forEach((value) => {
              total += value.volume;
            });
            setOrderBookState(
              "sells",
              r.map<TriElementFillComponent>((el) => {
                sum += el.volume;
                return {
                  column0: <span class="text-red">{format(1 / el.price, formatTemplate(precission))}</span>,
                  column1: format(el.volume, formatTemplate(precission)),
                  column2: format(sum, formatTemplate(precission)),
                  fill: sum / total,
                  fill_class: "bg-red",
                };
              })
            );
          });
      sells_events.onmessage = (ev) => {
        let total = 0,
          sum = 0;
        const sells = z.array(PriceLevel).parse(JSON.parse(ev.data)).reverse();
        sells.forEach((value) => {
          total += value.volume;
        });
        setOrderBookState(
          "sells",
          sells.map<TriElementFillComponent>((el) => {
            sum += el.volume;
            return {
              column0: <span class="text-red">{format(1 / el.price, formatTemplate(precission))}</span>,
              column1: format(el.volume, formatTemplate(precission)),
              column2: format(sum, formatTemplate(precission)),
              fill: sum / total,
              fill_class: "bg-red",
            };
          })
        );
      };

      const trades_events = new EventSource(
        `${api}/public/trades/sse?${params({
          quoteAssetId: quoteAsset.id,
          baseAssetId: baseAsset.id,
        })}`
      );
      trades_events.onopen = async () =>
        fetch(
          `${api}/public/trades?${params({
            quoteAssetId: quoteAsset.id,
            baseAssetId: baseAsset.id,
            limit: 1,
            offset: 0,
          })}`
        )
          .then((r) => r.json())
          .then((r) => z.array(Trade).parse(r)[0])
          .then((r) => {
            if (r) {
              if (r.quoteAssetId == quoteAsset.id && r.baseAssetId == baseAsset.id) {
                setOrderBookState("lastPrice", format(r.takerBaseVolume / r.takerQuoteVolume, formatTemplate(precission)));
              } else if (r.quoteAssetId == baseAsset.id && r.baseAssetId == quoteAsset.id) {
                setOrderBookState("lastPrice", format(r.takerQuoteVolume / r.takerBaseVolume, formatTemplate(precission)));
              }
            }
          });
      trades_events.onmessage = (ev) => {
        const last_trade = Trade.parse(JSON.parse(ev.data));
        if (last_trade.quoteAssetId == quoteAsset.id && last_trade.baseAssetId == baseAsset.id) {
          setOrderBookState("lastPrice", format(last_trade.takerBaseVolume / last_trade.takerQuoteVolume, formatTemplate(precission)));
        } else if (last_trade.quoteAssetId == baseAsset.id && last_trade.baseAssetId == quoteAsset.id) {
          setOrderBookState("lastPrice", format(last_trade.takerQuoteVolume / last_trade.takerBaseVolume, formatTemplate(precission)));
        }
      };
    }
  });

  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr_auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="p-4 font-sanspro text-orderbook-label font-semibold">Orderbook</div>
        <TriElementHeader
          class="py-[4px] px-[12px]"
          column0={<div class="text-left text-orderbook-sublabel">{`Price (${props.quoteAsset ? props.quoteAsset.symbol : "---"})`}</div>}
          column1={<div class="text-right text-orderbook-sublabel">{`Quantity (${props.baseAsset ? props.baseAsset.symbol : "---"})`}</div>}
          column2={<div class="text-right text-orderbook-sublabel">{`Total (${props.quoteAsset ? props.quoteAsset.symbol : "---"})`}</div>}
        />
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute top-0 left-0 right-0 bottom-0 flex flex-col-reverse overflow-clip">
          <Index each={orderBookState.sells}>
            {(element) => (
              <TriElementFill
                class="py-[4px] px-[12px] font-sanspro text-orderbook-item"
                column0={element().column0}
                column1={element().column1}
                column2={element().column2}
                fill={element().fill}
                fill_class={element().fill_class}
              />
            )}
          </Index>
        </div>
      </div>
      <div class="row-start-3 row-end-4 min-h-[50px]">
        <div class="flex h-full flex-col justify-center px-[12px]">
          <div class="text-orderbook-middle font-semibold ">{orderBookState.lastPrice}</div>
        </div>
      </div>
      <div class="relative row-start-4 row-end-5">
        <div class="absolute top-0 left-0 right-0 bottom-0 flex flex-col overflow-clip">
          <Index each={orderBookState.buys}>
            {(element) => (
              <TriElementFill
                class="py-[4px] px-[12px] font-sanspro text-orderbook-item"
                column0={element().column0}
                column1={element().column1}
                column2={element().column2}
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
