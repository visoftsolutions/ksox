import { Index, onCleanup, onMount, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { ValidateSignatureResponse } from "~/auth/mod";
import { Asset } from "~/types/asset";
import { Direction } from "../State";
import { Market } from "~/utils/providers/MarketProvider";
import { useAssets } from "~/utils/providers/AssetsProvider";
import { Uuid } from "~/types/primitives/uuid";
import { format } from "numerable";
import { formatTemplate } from "~/utils/precision";

interface TradeHistory {
  id: Uuid;
  order_time: Date;
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

  let events: EventSource | null = null;

  onMount(() => {
    if (props.market?.quote_asset && props.market?.base_asset && props.capacity && assets()) {
      const quote_asset = props.market?.quote_asset;
      const base_asset = props.market?.base_asset;
      const capacity = props.capacity;
    }
  });

  onCleanup(() => {
    events?.close();
  });

  return (
    <div class="row-start-3 row-end-4 overflow-auto">
      <Index each={Object.values(tradeHistory).sort((b, a) => a.order_time.getTime() - b.order_time.getTime())}>
        {(element, i) => (
          <div class={`grid grid-cols-7 items-center self-center px-[12px] py-[8px] text-state-item font-normal text-white ${i % 2 && "bg-gray-3"} `}>
            <div class="col-start-1 col-end-2 text-left">{element().order_time.toUTCString()}</div>
            <div class="col-start-2 col-end-3 text-center">{element().asset_pair[0]?.symbol + " / " + element().asset_pair[1]?.symbol}</div>
            <div class={`col-start-3 col-end-4 text-center ${element().direction == Direction.Buy ? "text-green" : "text-red"}`}>{element().direction}</div>
            <div class="col-start-4 col-end-5 text-center">{format(element().trade_price, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-5 col-end-6 text-center">{format(element().trade_value, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-6 col-end-7 text-center">{format(element().trade_qty, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-7 col-end-8 text-center">{format(element().fee, formatTemplate(props.precision ?? 2))}</div>
          </div>
        )}
      </Index>
    </div>
  );
}
