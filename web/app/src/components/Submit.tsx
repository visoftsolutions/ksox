import { ethers } from "ethers";
import { onCleanup, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import { ValidateSignatureResponse } from "~/auth/mod";
import { api } from "~/root";
import { Valut } from "~/types/valut";
import params from "~/utils/params";
import RectangularButton from "./Buttons/NavRectangularButton";
import BuyForm from "./Inputs/BuyForm";
import SellForm from "./Inputs/SellForm";
import { AssetResponse } from "./Markets";

export default function CreateSubmit(quote_asset?: AssetResponse, base_asset?: AssetResponse, session?: ValidateSignatureResponse, precision?: number) {
  return () => <Submit quote_asset={quote_asset} base_asset={base_asset} session={session} precision={precision} />;
}

export function Submit(props: { quote_asset?: AssetResponse; base_asset?: AssetResponse; session?: ValidateSignatureResponse; precision?: number }) {
  const [storeSubmit, setStoreSubmit] = createStore<{
    buy_available_balance?: bigint;
    sell_available_balance?: bigint;
  }>({
    buy_available_balance: undefined,
    sell_available_balance: undefined,
  });

  let buy_available_balance_events: EventSource | null = null;
  let sell_available_balance: EventSource | null = null;

  onMount(() => {
    if (props.session && props.quote_asset && props.base_asset && props.precision) {
      const quote_asset = props.quote_asset;
      const base_asset = props.base_asset;

      buy_available_balance_events = new EventSource(
        `${api}/private/balance/sse?${params({
          asset_id: quote_asset.id,
        })}`, { withCredentials: true }
      );
      buy_available_balance_events.onopen = async () => {
        await fetch(
          `${api}/private/balance?${params({
            asset_id: quote_asset.id,
          })}`,
          {
            method: "GET",
            credentials: "same-origin",
          }
        )
          .then((r) => r.json())
          .then((r) => Valut.parse(r))
          .then((r) => setStoreSubmit("buy_available_balance", r.balance));
      };
      buy_available_balance_events.onmessage = (ev) => {
        setStoreSubmit("buy_available_balance", Valut.parse(JSON.parse(ev.data)).balance);
      };

      sell_available_balance = new EventSource(
        `${api}/private/balance/sse?${params({
          asset_id: base_asset.id,
        })}`, { withCredentials: true }
      );
      sell_available_balance.onopen = async () => {
        await fetch(
          `${api}/private/balance?${params({
            asset_id: base_asset.id,
          })}`,
          {
            method: "GET",
            credentials: "same-origin",
          }
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

  onCleanup(() => {
    buy_available_balance_events?.close();
    sell_available_balance?.close();
  })

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
          {/* <div class="col-start-2 col-end-3 px-[12px]">
            <SellForm
              available_balance={storeSubmit.sell_available_balance}
              quote_asset={props.quote_asset}
              base_asset={props.base_asset}
              precision={props.precision}
            />
          </div> */}
        </div>
      </div>
    </div>
  );
}
