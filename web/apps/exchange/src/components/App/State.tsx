import { createSignal, Match, Show, Switch } from "solid-js";
import { SessionResponse } from "@packages/components/providers/SessionProvider/models";
import { Market } from "~/components/providers/MarketProvider";
import NavButton from "./Buttons/NavButton";
import CreateOpenOrders from "./State/OpenOrders";
import CreateOrderHistory from "./State/OrderHistory";
import CreateTradeHistory from "./State/TradeHistory";

enum StateTabs {
  OpenOrders,
  OrderHistory,
  TradeHistory,
}

export default function CreateState(
  market?: Market,
  session?: SessionResponse,
  precision?: number,
  capacity?: number,
) {
  return () => (
    <Show when={session && precision} fallback={<State />}>
      <State
        market={market}
        session={session}
        precision={precision}
        capacity={capacity}
      />
    </Show>
  );
}

export function State(props: {
  market?: Market;
  session?: SessionResponse;
  precision?: number;
  capacity?: number;
}) {
  const [tab, setTab] = createSignal<StateTabs>(StateTabs.OpenOrders);

  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_36px_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="inline-grid grid-cols-[auto_auto_auto] grid-rows-1 gap-[1px]">
          <NavButton
            class="col-start-1 col-end-2"
            highlighted={tab() == StateTabs.OpenOrders}
            onClick={() => {
              setTab(StateTabs.OpenOrders);
            }}
          >
            <span class="text-state-label">Open Orders</span>
          </NavButton>
          <NavButton
            class="col-start-2 col-end-3"
            highlighted={tab() == StateTabs.OrderHistory}
            onClick={() => {
              setTab(StateTabs.OrderHistory);
            }}
          >
            <span class="text-state-label">Order History</span>
          </NavButton>
          <NavButton
            class="col-start-3 col-end-4"
            highlighted={tab() == StateTabs.TradeHistory}
            onClick={() => {
              setTab(StateTabs.TradeHistory);
            }}
          >
            <span class="text-state-label">Trade History</span>
          </NavButton>
        </div>
      </div>

      <Switch>
        <Match when={tab() == StateTabs.OpenOrders}>
          {CreateOpenOrders(
            props.market,
            props.session,
            props.precision,
            props.capacity,
          )()}
        </Match>
        <Match when={tab() == StateTabs.OrderHistory}>
          {CreateOrderHistory(
            props.market,
            props.session,
            props.precision,
            props.capacity,
          )()}
        </Match>
        <Match when={tab() == StateTabs.TradeHistory}>
          {CreateTradeHistory(
            props.market,
            props.session,
            props.precision,
            props.capacity,
          )()}
        </Match>
      </Switch>
    </div>
  );
}
