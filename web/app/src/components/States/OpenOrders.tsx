import { createSignal, Index, onCleanup, onMount, Show } from "solid-js";
import { createStore, produce } from "solid-js/store";
import { ValidateSignatureResponse } from "~/auth/mod";
import { Asset } from "~/types/asset";
import { fromWei } from "~/utils/converters/wei";
import { Direction } from "../State";
import { api } from "~/root";
import params from "~/utils/params";
import { Order } from "~/types/order";
import { z } from "zod";
import { Uuid } from "~/types/primitives/uuid";
import { useAssets } from "~/utils/providers/AssetsProvider";
import { Market } from "~/utils/providers/MarketProvider";
import { format } from "numerable";
import { formatTemplate } from "~/utils/precision";

interface OpenOrder {
  id: Uuid;
  order_time: Date;
  asset_pair: [Asset | undefined, Asset | undefined];
  direction: Direction;
  order_price: number;
  order_value: number;
  order_qty: number;
  filled_qty: number;
}

export default function CreateOpenOrders(market?: Market, session?: ValidateSignatureResponse, precision?: number, capacity?: number) {
  return () => (
    <>
      <div class="row-start-2 row-end-3">
        <div class="grid grid-cols-8 items-center self-center px-[8px] py-[8px] text-state-sublabel font-semibold text-gray-4">
          <div class="col-start-1 col-end-2 text-left">Order Time</div>
          <div class="col-start-2 col-end-3 text-center">Asset Pair</div>
          <div class="col-start-3 col-end-4 text-center">Direction</div>
          <div class="col-start-4 col-end-5 text-center">Order Price</div>
          <div class="col-start-5 col-end-6 text-center">Order Value</div>
          <div class="col-start-6 col-end-7 text-center">Order Qty</div>
          <div class="col-start-7 col-end-8 text-center">Filled Qty</div>
          <div class="col-start-8 col-end-9 text-right">Action</div>
        </div>
      </div>
      <Show when={market && session && precision}>
        <OpenOrders market={market} session={session} precision={precision} capacity={capacity} />
      </Show>
    </>
  );
}

export function OpenOrders(props: { market?: Market; session?: ValidateSignatureResponse; precision?: number; capacity?: number }) {
  const [store, setStore] = createSignal<Map<Uuid, OpenOrder>>(new Map());
  const assets = useAssets();

  let events: EventSource | null = null;

  onMount(() => {
    if (props.market && props.precision && props.capacity && assets()) {
      const market = props.market;
      const capacity = props.capacity;

      events = new EventSource(`${api}/private/active/sse`, { withCredentials: true });
      events.onopen = async () =>
        await fetch(
          `${api}/private/active?${params({
            limit: capacity,
            offset: 0,
          })}`,
          { credentials: "same-origin" }
        )
          .then((r) => r.json())
          .then((r) => z.array(Order).parse(r))
          .then((r) => {
            return r.map<OpenOrder>((el) => {
              return {
                id: el.id,
                order_time: el.created_at,
                asset_pair: [assets().get(el.base_asset_id), assets().get(el.quote_asset_id)],
                direction: el.quote_asset_id == market?.quote_asset?.id ? Direction.Buy : Direction.Sell,
                order_price: fromWei(el.base_asset_volume / el.quote_asset_volume),
                order_value: fromWei(el.quote_asset_volume),
                order_qty: fromWei(el.base_asset_volume),
                filled_qty: fromWei(el.quote_asset_volume - el.quote_asset_volume_left),
              };
            });
          })
          .then((r) => setStore(produce((s) => r.forEach((e) => s.set(e.id, e)))));
      events.onmessage = (ev) => {
        const el = Order.parse(JSON.parse(ev.data));
        const element: OpenOrder = {
          id: el.id,
          order_time: el.created_at,
          asset_pair: [assets().get(el.base_asset_id), assets().get(el.quote_asset_id)],
          direction: el.quote_asset_id == market?.quote_asset?.id ? Direction.Buy : Direction.Sell,
          order_price: fromWei(el.base_asset_volume / el.quote_asset_volume),
          order_value: fromWei(el.quote_asset_volume),
          order_qty: fromWei(el.base_asset_volume),
          filled_qty: fromWei(el.quote_asset_volume - el.quote_asset_volume_left),
        };
        setStore(produce((s) => s.set(element.id, element)));
      };
    }
  });

  onCleanup(() => {
    events?.close();
  });

  return (
    <div class="row-start-2 row-end-3 overflow-auto">
      <Index each={Array.from(store().values())}>
        {(element, i) => (
          <div class={`grid grid-cols-8 items-center self-center px-[12px] py-[8px] text-state-item font-normal text-white ${i % 2 ? "bg-gray-3" : ""} `}>
            <div class="col-start-1 col-end-2 text-left">{element().order_time.toUTCString()}</div>
            <div class="col-start-2 col-end-3 text-center">{element().asset_pair[0]?.symbol + " / " + element().asset_pair[1]?.symbol}</div>
            <div class={`col-start-3 col-end-4 text-center ${element().direction == Direction.Buy ? "text-red" : "text-green"}`}>{element().direction}</div>
            <div class="col-start-4 col-end-5 text-center">{format(element().order_price, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-5 col-end-6 text-center">{format(element().order_value, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-6 col-end-7 text-center">{format(element().order_qty, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-7 col-end-8 text-center">{format(element().filled_qty, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-8 col-end-9 flex justify-end">Action</div>
          </div>
        )}
      </Index>
    </div>
  );
}
