import { Index, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import { TriElementComponent } from "~/components/TriElement/TriElement";
import TriElement from "~/components/TriElement/TriElement";
import TriElementHeader from "~/components/TriElement/TriElementHeader";
import { Trade } from "~/types/trade";
import params from "~/utils/params";
import { z } from "zod";
import { format } from "numerable";
import { Asset } from "~/types/asset";
import { formatTemplate } from "~/utils/precision";
import { api } from "~/root";

export function CreateTrades(baseAssetId?: string, quoteAssetId?: string, precision?: number, capacity?: number) {
  return () => <Trades baseAssetId={baseAssetId} quoteAssetId={quoteAssetId} precission={precision} capacity={capacity} />;
}

export default function Trades(props: { quoteAsset?: Asset; baseAsset?: Asset; precission?: number; capacity?: number }) {
  const [tradesState, setTradesState] = createStore<{ trades: TriElementComponent[] }>({ trades: [] });

  onMount(async () => {
    if (props.quoteAsset && props.baseAsset && props.precission && props.capacity) {
      const quoteAsset = props.quoteAsset;
      const baseAsset = props.baseAsset;
      const precission = props.precission;
      const capacity = props.capacity;

      const events = new EventSource(
        `${api}/public/trades/sse?${params({
          quoteAssetId: quoteAsset.id,
          baseAssetId: baseAsset.id,
        })}`
      );
      events.onopen = async () =>
        fetch(
          `${api}/public/trades?${params({
            quoteAssetId: quoteAsset.id,
            baseAssetId: baseAsset.id,
            limit: capacity,
            offset: 0,
          })}`
        )
          .then((r) => r.json())
          .then((r) => z.array(Trade).parse(r))
          .then((r) => {
            return r.map<TriElementComponent>((el) => {
              const price =
                el.quoteAssetId == quoteAsset.id && el.baseAssetId == baseAsset.id
                  ? el.takerBaseVolume / el.takerQuoteVolume
                  : el.takerQuoteVolume / el.takerBaseVolume;
              return {
                column0: (
                  <span class={`${el.quoteAssetId == quoteAsset.id && el.baseAssetId == baseAsset.id ? "text-green" : "text-red"}`}>
                    {format(price, formatTemplate(precission))}
                  </span>
                ),
                column1: format(el.takerBaseVolume, formatTemplate(precission)),
                column2: el.createdAt.toLocaleTimeString(),
              };
            });
          })
          .then((r) => setTradesState("trades", r.slice(0, props.capacity)));
      events.onmessage = (ev) => {
        const last_trade = Trade.parse(JSON.parse(ev.data));
        const price =
          last_trade.quoteAssetId == quoteAsset.id && last_trade.baseAssetId == baseAsset.id
            ? last_trade.takerBaseVolume / last_trade.takerQuoteVolume
            : last_trade.takerQuoteVolume / last_trade.takerBaseVolume;
        const trade = {
          column0: (
            <span class={`${last_trade.quoteAssetId == quoteAsset.id && last_trade.baseAssetId == baseAsset.id ? "text-green" : "text-red"}`}>
              {format(price, formatTemplate(precission))}
            </span>
          ),
          column1: format(last_trade.takerBaseVolume, formatTemplate(precission)),
          column2: last_trade.createdAt.toLocaleTimeString(),
        };
        setTradesState("trades", (prev) => [trade, ...prev].slice(0, props.capacity));
      };
    }
  });

  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="p-4 font-sanspro text-trades-label font-semibold">Trades</div>
        <TriElementHeader
          class="px-[12px] py-[4px]"
          column0={<div class="text-left text-trades-sublabel">{`Price (${props.quoteAsset ? props.quoteAsset.symbol : "---"})`}</div>}
          column1={<div class="text-right text-trades-sublabel">{`Quantity (${props.baseAsset ? props.baseAsset.symbol : "---"})`}</div>}
          column2={<div class="text-right text-trades-sublabel">{"Time"}</div>}
        />
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col overflow-clip">
          <Index each={tradesState.trades}>
            {(element, i) => (
              <TriElement
                class={`px-[12px] py-[4px] font-sanspro text-trades-item ${i % 2 ? "bg-gray-3" : ""}`}
                column0={element().column0}
                column1={element().column1}
                column2={element().column2}
              />
            )}
          </Index>
        </div>
      </div>
    </div>
  );
}
