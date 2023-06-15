import { batch, Index, onCleanup, onMount, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { Asset } from "@web/types/asset";
import { api, base } from "~/root";
import params from "@web/utils/params";
import { PrivateOrder } from "@web/types/order";
import { z } from "zod";
import { Uuid } from "@web/types/primitives/uuid";
import { useAssets } from "~/components/providers/AssetsProvider";
import { Market } from "~/components/providers/MarketProvider";
import { format } from "numerable";
import { formatTemplate } from "@web/utils/precision";
import { joinPaths } from "solid-start/islands/server-router";
import { ev, finv, fmul } from "@web/types/primitives/fraction";
import { Direction } from "@web/types/primitives/direction";
import subscribeEvents from "@web/utils/subscribeEvents";
import { SessionResponse } from "@web/components/providers/SessionProvider/models";

interface OpenOrder {
  id: Uuid;
  is_active: boolean;
  order_time: Date;
  asset_pair: [Asset | undefined, Asset | undefined];
  direction: Direction;
  order_price: number;
  order_value: number;
  order_qty: number;
  filled_qty: number;
}

export default function CreateOpenOrders(market?: Market, session?: SessionResponse, precision?: number, capacity?: number) {
  return () => (
    <>
      <div class="row-start-2 row-end-3 grid grid-cols-8 items-center self-center px-3 text-stateHeader font-semibold text-gray-4">
        <div class="col-start-1 col-end-2 text-left">Order Time</div>
        <div class="col-start-2 col-end-3 text-center">Asset Pair</div>
        <div class="col-start-3 col-end-4 text-center">Direction</div>
        <div class="col-start-4 col-end-5 text-center">Order Price</div>
        <div class="col-start-5 col-end-6 text-center">Order Value</div>
        <div class="col-start-6 col-end-7 text-center">Order Qty</div>
        <div class="col-start-7 col-end-8 text-center">Filled Qty</div>
        <div class="col-start-8 col-end-9 text-right">Action</div>
      </div>
      <Show when={market && session && precision}>
        <OpenOrders market={market} session={session} precision={precision} capacity={capacity} />
      </Show>
    </>
  );
}

export function OpenOrders(props: { market?: Market; session?: SessionResponse; precision?: number; capacity?: number }) {
  const assets = useAssets();
  const [openOrders, setOpenOrders] = createStore<{ [key: Uuid]: OpenOrder }>({});

  let eventsource: EventSource | undefined;

  onMount(async () => {
    if (props.market?.quote_asset && props.market?.base_asset && props.capacity && assets()) {
      const capacity = props.capacity;

      const convertOpenOrder = (order: PrivateOrder): OpenOrder => {
        const price = ev(order.price);
        const quote_asset_volume = ev(order.quote_asset_volume);
        const quote_asset_volume_left = ev(order.quote_asset_volume_left);
        const filled_qty = (quote_asset_volume - quote_asset_volume_left) / price;
        const base_asset_volume = ev(fmul(order.quote_asset_volume, finv(order.price)));
        const asset_pair: [Asset | undefined, Asset | undefined] = [assets().get(order.base_asset_id), assets().get(order.quote_asset_id)];
        return {
          id: order.id,
          is_active: order.is_active,
          order_time: order.created_at,
          asset_pair: asset_pair,
          direction: order.direction,
          order_price: price,
          order_value: quote_asset_volume,
          order_qty: base_asset_volume,
          filled_qty: filled_qty,
        };
      };

      eventsource = await subscribeEvents(`${api}/private/active`, params({ limit: capacity, offset: 0 }), params({}), (data) => {
        batch(() =>
          z
            .array(PrivateOrder)
            .parse(data)
            .map(convertOpenOrder)
            .forEach((e) => setOpenOrders({ [e.id]: e.is_active ? e : undefined }))
        );
      });
    }
  });

  onCleanup(() => {
    eventsource?.close();
  });

  return (
    <div class="row-start-3 row-end-4 overflow-auto">
      <Index each={Object.values(openOrders).sort((b, a) => a.order_time.getTime() - b.order_time.getTime())}>
        {(element, i) => (
          <div class={`grid grid-cols-8 items-center self-center px-[12px] py-[8px] text-state-item font-normal text-white ${i % 2 && "bg-gray-3"} `}>
            <div class="col-start-1 col-end-2 text-left">{element().order_time.toUTCString()}</div>
            <div class="col-start-2 col-end-3 text-center">{element().asset_pair[0]?.symbol + " / " + element().asset_pair[1]?.symbol}</div>
            <div class={`col-start-3 col-end-4 text-center ${element().direction == Direction.Enum.buy ? "text-green" : "text-red"}`}>
              {element().direction}
            </div>
            <div class="col-start-4 col-end-5 text-center">{format(element().order_price, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-5 col-end-6 text-center">{format(element().order_value, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-6 col-end-7 text-center">{format(element().order_qty, formatTemplate(props.precision ?? 2))}</div>
            <div class="col-start-7 col-end-8 text-center">{format(element().filled_qty, formatTemplate(props.precision ?? 2))}</div>
            <div
              class="col-start-8 col-end-9 flex cursor-pointer justify-end"
              onClick={async () => {
                await fetch(
                  `${api}/private/cancel?${params({
                    order_id: element().id,
                  })}`,
                  { method: "DELETE", credentials: "same-origin" }
                ).then((r) => r.text());
              }}
            >
              <img src={joinPaths(base, "/gfx/cancel.svg")} class="h-[12px] w-[12px]" />
            </div>
          </div>
        )}
      </Index>
    </div>
  );
}
