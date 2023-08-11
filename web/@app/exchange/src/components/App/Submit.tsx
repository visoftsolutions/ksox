import { onCleanup, onMount, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { SessionResponse } from "@web/components/providers/SessionProvider/models";
import { api } from "~/root";
import { Valut } from "@web/types/valut";
import params from "@web/utils/params";
import { Market } from "~/components/providers/MarketProvider";
import RectangularButton from "./Buttons/NavButton";
import BuyForm from "./Submit/BuyForm";
import SellForm from "./Submit/SellForm";
import { Fraction } from "@web/types/primitives/fraction";
import subscribeEvents from "@web/utils/subscribeEvents";
import { Value } from "@web/types/primitives/value";

export default function CreateSubmit(market: Market, session?: SessionResponse, precision?: number) {
  return () => (
    <Show when={market && market.quote_asset && market.base_asset && session && precision} fallback={<Submit />}>
      <Submit market={market} session={session} precision={precision} />
    </Show>
  );
}

export function Submit(props: { market?: Market; session?: SessionResponse; precision?: number }) {
  const [storeSubmit, setStoreSubmit] = createStore<{
    buy_available_balance?: Value;
    sell_available_balance?: Value;
  }>({
    buy_available_balance: undefined,
    sell_available_balance: undefined,
  });

  let eventsource_balance_buy: EventSource | undefined;
  let eventsource_balance_sell: EventSource | undefined;

  onMount(async () => {
    if (props.session && props.market?.quote_asset && props.market?.base_asset && props.precision) {
      const quote_asset = props.market?.quote_asset;
      const base_asset = props.market?.base_asset;

      eventsource_balance_buy = await subscribeEvents(
        `${api}/private/balance`,
        params({ asset_id: quote_asset.id }),
        params({ asset_id: quote_asset.id }),
        (data) => {
          setStoreSubmit("buy_available_balance", Valut.parse(data).balance);
        }
      );

      eventsource_balance_sell = await subscribeEvents(
        `${api}/private/balance`,
        params({ asset_id: base_asset.id }),
        params({ asset_id: base_asset.id }),
        (data) => {
          setStoreSubmit("sell_available_balance", Valut.parse(data).balance);
        }
      );
    }
  });

  onCleanup(() => {
    eventsource_balance_buy?.close();
    eventsource_balance_sell?.close();
  });

  return (
    <div class="grid h-full grid-rows-[auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="inline-grid gap-1">
          <RectangularButton class="col-start-1 col-end-2" highlighted={false}>
            <span class="text-openorders-label">Limit</span>
          </RectangularButton>
        </div>
      </div>
      <div class="row-start-2 row-end-3 overflow-auto py-[8px]">
        <div class="grid h-full grid-cols-2 grid-rows-1">
          <div class="col-start-1 col-end-2 px-[12px] ">
            <BuyForm available_balance={storeSubmit.buy_available_balance} market={props.market} precision={props.precision} />
          </div>
          <div class="col-start-2 col-end-3 px-[12px]">
            <SellForm available_balance={storeSubmit.sell_available_balance} market={props.market} precision={props.precision} />
          </div>
        </div>
      </div>
    </div>
  );
}
