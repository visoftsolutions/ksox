import { createEffect } from "solid-js";
import { createStore } from "solid-js/store";
import { api } from "~/root";
import { Valut } from "~/types/valut";
import params from "~/utils/params";
import RectangularButton from "./Buttons/NavRectangularButton";
import { useSession } from "./Buttons/WalletButton";
import BuyForm from "./Inputs/BuyForm";
import SellForm from "./Inputs/SellForm";
import { AssetResponse } from "./Markets";

export default function CreateSubmit(quote_asset?: AssetResponse, base_asset?: AssetResponse, precision?: number) {
  return () => <Submit quote_asset={quote_asset} base_asset={base_asset} precision={precision} />;
}

export function Submit(props: { quote_asset?: AssetResponse; base_asset?: AssetResponse; precision?: number }) {
  const [storeSubmit, setStoreSubmit] = createStore<{
    buy_available_balance?: number;
    sell_available_balance?: number;
  }>({
    buy_available_balance: undefined,
    sell_available_balance: undefined,
  });

  const session = useSession();

  createEffect(() => {
    if (session && session() && props.quote_asset && props.base_asset && props.precision) {
      const quote_asset = props.quote_asset;
      const base_asset = props.base_asset;

      const buy_available_balance_events = new EventSource(
        `${api}/private/balance/sse?${params({
          asset_id: quote_asset.id,
        })}`
      );
      buy_available_balance_events.onopen = async () => {
        await fetch(
          `${api}/private/balance?${params({
            asset_id: quote_asset.id,
          })}`
        )
          .then((r) => r.json())
          .then((r) => Valut.parse(r))
          .then((r) => setStoreSubmit("buy_available_balance", r.balance));
      };
      buy_available_balance_events.onmessage = (ev) => {
        setStoreSubmit("buy_available_balance", Valut.parse(JSON.parse(ev.data)).balance);
      };

      const sell_available_balance = new EventSource(
        `${api}/private/balance/sse?${params({
          asset_id: base_asset.id,
        })}`
      );
      sell_available_balance.onopen = async () => {
        await fetch(
          `${api}/private/balance?${params({
            asset_id: base_asset.id,
          })}`
        )
          .then((r) => r.json())
          .then((r) => Valut.parse(r))
          .then((r) => setStoreSubmit("sell_available_balance", r.balance));
      };
      sell_available_balance.onmessage = (ev) => {
        setStoreSubmit("sell_available_balance", Valut.parse(JSON.parse(ev.data)).balance);
      };
    }
  });

  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr]">
      <div class="row-start-1 row-end-2 px-[4px] pt-[12px]">
        <div class="inline-grid grid-cols-[auto_auto_auto] grid-rows-1 gap-1">
          <RectangularButton class="col-start-1 col-end-2" highlighted={false}>
            <span class="text-openorders-label">Limit</span>
          </RectangularButton>
        </div>
      </div>
      <div class="row-start-2 row-end-3 overflow-auto py-[8px]">
        <div class="grid h-full grid-cols-2 grid-rows-1">
          <div class="col-start-1 col-end-2 px-[12px] ">
            <BuyForm
              available_balance={storeSubmit.buy_available_balance}
              quote_asset={props.quote_asset}
              base_asset={props.base_asset}
              precision={props.precision}
            />
          </div>
          <div class="col-start-2 col-end-3 px-[12px]">
            <SellForm
              available_balance={storeSubmit.sell_available_balance}
              quote_asset={props.quote_asset}
              base_asset={props.base_asset}
              precision={props.precision}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
