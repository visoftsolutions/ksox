import { Index } from "solid-js";
import { createStore } from "solid-js/store";
import RectangularButton from "./Buttons/NavRectangularButton";
import StateActionCircularButton from "./Buttons/StateActionCircularButton";

export enum OrderSide {
  Buy = "buy",
  Sell = "sell",
}

enum StateTabs {
  OpenOrders = "open_orders",
  OrderHistory = "order_history",
  TradeHistory = "trade_history",
}

interface OpenOrders {
  order_time: Date;
  asset_pair: [string, string];
  order_side: OrderSide;
  price: string;
  quantity: string;
  filled_quantity: string;
}

interface OrderHistory {
  order_time: Date;
  asset_pair: [string, string];
  order_side: OrderSide;
  price: string;
  quantity: string;
  filled_quantity: string;
}

interface TradeHistory {
  order_time: Date;
  asset_pair: [string, string];
  order_side: OrderSide;
  price: string;
  quantity: string;
  filled_quantity: string;
}

export interface StateComponent {
  tab: StateTabs;
  open_orders: OpenOrders[];
  order_history: OrderHistory[];
  trade_history: TradeHistory[];
}

export const [store, setStore] = createStore<StateComponent>({ tab: StateTabs.OpenOrders, open_orders: [], order_history: [], trade_history: [] });

export default function State() {
  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr]">
      <div class="row-start-1 row-end-2 px-[4px] pt-[12px]">
        <div class="inline-grid grid-cols-[auto_auto_auto] grid-rows-1 gap-1">
          <RectangularButton
            class="col-start-1 col-end-2"
            highlighted={store.tab == StateTabs.OpenOrders}
            onClick={() => {
              setStore({ tab: StateTabs.OpenOrders });
            }}
          >
            <span class="text-state-label">Open Orders</span>
          </RectangularButton>
          <RectangularButton
            class="col-start-2 col-end-3"
            highlighted={store.tab == StateTabs.OrderHistory}
            onClick={() => {
              setStore({ tab: StateTabs.OrderHistory });
            }}
          >
            <span class="text-state-label">Order History</span>
          </RectangularButton>
          <RectangularButton
            class="col-start-3 col-end-4"
            highlighted={store.tab == StateTabs.TradeHistory}
            onClick={() => {
              setStore({ tab: StateTabs.TradeHistory });
            }}
          >
            <span class="text-state-label">Trade History</span>
          </RectangularButton>
        </div>
        <div class="grid grid-cols-7 items-center self-center px-[8px] py-[8px] text-state-sublabel font-semibold text-gray-4">
          <div class="col-start-1 col-end-2 text-left">Time</div>
          <div class="col-start-2 col-end-3 text-center">Spot Pair</div>
          <div class="col-start-3 col-end-4 text-center">Order Side</div>
          <div class="col-start-4 col-end-5 text-center">Price</div>
          <div class="col-start-5 col-end-6 text-center">Quantity</div>
          <div class="col-start-6 col-end-7 text-center">Filled Quantity</div>
          <div class="col-start-7 col-end-8 text-right">Action</div>
        </div>
      </div>

      <div class="row-start-2 row-end-3 overflow-auto">
        <Index each={store[store.tab]}>
          {(element, i) => (
            <div class={`grid grid-cols-7 items-center self-center px-[12px] py-[8px] text-state-item font-normal text-white ${i % 2 ? "bg-gray-3" : ""} `}>
              <div class="col-start-1 col-end-2 text-left">{element().order_time.toUTCString()}</div>
              <div class="col-start-2 col-end-3 text-center">{element().asset_pair[0] + " / " + element().asset_pair[1]}</div>
              <div class={`col-start-3 col-end-4 text-center ${element().order_side == OrderSide.Buy ? "text-red" : "text-green"}`}>{element().order_side}</div>
              <div class="col-start-4 col-end-5 text-center">{element().price}</div>
              <div class="col-start-5 col-end-6 text-center">{element().quantity}</div>
              <div class="col-start-6 col-end-7 text-center">{element().filled_quantity}</div>
              <div class="col-start-7 col-end-8 flex justify-end">
                <StateActionCircularButton class="ml-2 bg-green" />
                <StateActionCircularButton class="ml-2 bg-gray-4" />
                <StateActionCircularButton class="ml-2 bg-red" />
              </div>
            </div>
          )}
        </Index>
      </div>
    </div>
  );
}
