import { Index, onMount, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { ValidateSignatureResponse } from "~/auth/mod";
import { Asset } from "~/types/asset";
import { fromWei } from "~/utils/converters/wei";
import { Direction } from "../State";
import { Market } from "~/utils/providers/MarketProvider";

interface TradeHistory {
  order_time: Date;
  asset_pair: [Asset, Asset];
  direction: Direction;
  trade_price: number;
  trade_value: bigint;
  trade_qty: bigint;
  fee: number;
}

export default function CreateTradeHistory(market?: Market, session?: ValidateSignatureResponse, precision?: number, capacity?: number) {
  return () => (
    <>
      <div class="row-start-2 row-end-3">
        <div class="grid grid-cols-8 items-center self-center px-[8px] py-[8px] text-state-sublabel font-semibold text-gray-4">
          <div class="col-start-1 col-end-2 text-left">Order Time</div>
          <div class="col-start-2 col-end-3 text-center">Asset Pair</div>
          <div class="col-start-3 col-end-4 text-center">Direction</div>
          <div class="col-start-4 col-end-5 text-center">Trade Price</div>
          <div class="col-start-5 col-end-6 text-center">Trade Value</div>
          <div class="col-start-6 col-end-7 text-center">Trade Qty</div>
          <div class="col-start-7 col-end-8 text-center">Fee</div>
          <div class="col-start-8 col-end-9 text-right">Action</div>
        </div>
      </div>
      <Show when={session && precision}>
        <TradeHistory session={session} precision={precision} capacity={capacity} />
      </Show>
    </>
  );
}

export function TradeHistory(props: { market?: Market; session?: ValidateSignatureResponse; precision?: number; capacity?: number }) {
  const [store, setStore] = createStore<TradeHistory[]>([]);

  return (
    <div class="row-start-2 row-end-3 overflow-auto">
      <Index each={store}>
        {(element, i) => (
          <div class={`grid grid-cols-8 items-center self-center px-[12px] py-[8px] text-state-item font-normal text-white ${i % 2 ? "bg-gray-3" : ""} `}>
            <div class="col-start-1 col-end-2 text-left">{element().order_time.toUTCString()}</div>
            <div class="col-start-2 col-end-3 text-center">{element().asset_pair[0].symbol + " / " + element().asset_pair[1].symbol}</div>
            <div class={`col-start-3 col-end-4 text-center ${element().direction == Direction.Buy ? "text-red" : "text-green"}`}>{element().direction}</div>
            <div class="col-start-4 col-end-5 text-center">{element().trade_price}</div>
            <div class="col-start-5 col-end-6 text-center">{fromWei(element().trade_value)}</div>
            <div class="col-start-6 col-end-7 text-center">{fromWei(element().trade_qty)}</div>
            <div class="col-start-7 col-end-8 text-center">{element().fee}</div>
            <div class="col-start-8 col-end-9 flex justify-end">Action</div>
          </div>
        )}
      </Index>
    </div>
  );
}
