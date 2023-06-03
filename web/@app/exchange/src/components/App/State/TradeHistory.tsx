import { batch, Index, onCleanup, onMount, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { ValidateSignatureResponse } from "@web/components/providers/SessionProvider/models";
import { Asset } from "@web/types/asset";
import { Market } from "~/components/providers/MarketProvider";
import { useAssets } from "~/components/providers/AssetsProvider";
import { Uuid } from "@web/types/primitives/uuid";
import { format } from "numerable";
import { formatTemplate } from "@web/utils/precision";
import { Direction } from "@web/types/primitives/direction";
import { api } from "~/root";
import params from "@web/utils/params";
import { PrivateTrade } from "@web/types/trade";
import { z } from "zod";
import { ev } from "@web/types/primitives/fraction";
import subscribeEvents from "@web/utils/subscribeEvents";

interface TradeHistory {
  id: Uuid;
  trade_time: Date;
  asset_pair: [Asset | undefined, Asset | undefined];
  direction: Direction;
  trade_price: number;
  trade_value: number;
  trade_qty: number;
  fee: number;
}

export default function CreateTradeHistory(market?: Market, session?: ValidateSignatureResponse, precision?: number, capacity?: number) {
  return () => (
    <>
      <div class="row-start-2 row-end-3 grid grid-cols-7 items-center self-center px-3 text-stateHeader font-semibold text-gray-4">
        <div class="col-start-1 col-end-2 text-left">Trade Time</div>
        <div class="col-start-2 col-end-3 text-center">Asset Pair</div>
        <div class="col-start-3 col-end-4 text-center">Direction</div>
        <div class="col-start-4 col-end-5 text-center">Trade Price</div>
        <div class="col-start-5 col-end-6 text-center">Trade Value</div>
        <div class="col-start-6 col-end-7 text-center">Trade Qty</div>
        <div class="col-start-7 col-end-8 text-right">Fee</div>
      </div>
      <Show when={market && session && precision}>
        <TradeHistory market={market} session={session} precision={precision} capacity={capacity} />
      </Show>
    </>
  );
}

export function TradeHistory(props: { market?: Market; session?: ValidateSignatureResponse; precision?: number; capacity?: number }) {
  const assets = useAssets();
  const [tradeHistory, setTradeHistory] = createStore<{ [key: Uuid]: TradeHistory }>({});

  let eventsource: EventSource | undefined;

  onMount(async () => {
    if (props.market?.quote_asset && props.market?.base_asset && props.capacity && assets()) {
      const capacity = props.capacity;

      const convertTradeHistory = (trade: PrivateTrade): TradeHistory => {
        const price = ev(trade.price);
        const quote_asset_volume = ev(trade.taker_quote_volume);
        const base_asset_volume = ev(trade.taker_base_volume);
        const asset_pair: [Asset | undefined, Asset | undefined] = [assets().get(trade.base_asset_id), assets().get(trade.quote_asset_id)];
        const fee = (ev(trade.maker_quote_volume) - base_asset_volume) * price;
        return {
          id: trade.id,
          trade_time: trade.created_at,
          asset_pair: asset_pair,
          direction: trade.direction,
          trade_price: price,
          trade_value: quote_asset_volume,
          trade_qty: base_asset_volume,
          fee: fee,
        };
      };

      eventsource = await subscribeEvents(`${api}/private/trades`, params({ limit: capacity, offset: 0 }), params({}), (data) => {
        batch(() =>
          z
            .array(PrivateTrade)
            .parse(data)
            .map(convertTradeHistory)
            .forEach((e) => setTradeHistory({ [e.id]: e }))
        );
      });
    }
  });

  onCleanup(() => {
    eventsource?.close();
  });

  return (
    <div class="row-start-3 row-end-4 overflow-auto">
      <Index each={Object.values(tradeHistory).sort((b, a) => a.trade_time.getTime() - b.trade_time.getTime())}>
        {(element, i) => (
          <div class={`grid grid-cols-7 items-center self-center px-[12px] py-[8px] text-state-item font-normal text-white ${i % 2 && "bg-gray-3"} `}>
            <div class="col-start-1 col-end-2 text-left">{element().trade_time.toUTCString()}</div>
            <div class="col-start-2 col-end-3 text-center">{element().asset_pair[0]?.symbol + " / " + element().asset_pair[1]?.symbol}</div>
            <div class={`col-start-3 col-end-4 text-center ${element().direction == Direction.Enum.buy ? "text-green" : "text-red"}`}>
              {element().direction}
            </div>
            <div class="col-start-4 col-end-5 text-center">{format(element().trade_price, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-5 col-end-6 text-center">{format(element().trade_value, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-6 col-end-7 text-center">{format(element().trade_qty, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-7 col-end-8 text-right">{format(element().fee, formatTemplate(props.precision ?? 2))}</div>
          </div>
        )}
      </Index>
    </div>
  );
}
