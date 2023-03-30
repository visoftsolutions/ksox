import { createEffect } from "solid-js";
import { createStore } from "solid-js/store";
import { api } from "~/root";
import { Asset } from "~/types/asset";
import { Valut } from "~/types/valut";
import params from "~/utils/params";
import RectangularButton from "./Buttons/NavRectangularButton";
import { useSession } from "./Buttons/WalletButton";
import BuyForm from "./Inputs/BuyForm";
import SellForm from "./Inputs/SellForm";

export default function CreateSubmit(baseAssetId?: string, quoteAssetId?: string, precision?: number) {

  return () => {
    return <Submit quoteAsset={quoteAsset} baseAsset={baseAsset} precision={precision} />;
  };
}

export function Submit(props: { quoteAsset?: Asset; baseAsset?: Asset; precision?: number }) {
  const [storeSubmit, setStoreSubmit] = createStore<{
    buy_availableBalance: number | null;
    sell_availableBalance: number | null;
  }>({
    buy_availableBalance: null,
    sell_availableBalance: null,
  });

  const session = useSession();

  createEffect(async () => {
    if (session && session() && props.quoteAsset && props.baseAsset && props.precision) {
      const quoteAsset = props.quoteAsset;
      const baseAsset = props.baseAsset;

      const buy_availableBalance_events = new EventSource(
        `${api}/private/balance/sse?${params({
          asset_id: quoteAsset.id,
        })}`
      );
      buy_availableBalance_events.onopen = async () => {
        await fetch(
          `${api}/private/balance?${params({
            asset_id: quoteAsset.id,
          })}`
        )
          .then((r) => r.json())
          .then((r) => Valut.parse(r))
          .then((r) => setStoreSubmit("buy_availableBalance", r.balance));
      };
      buy_availableBalance_events.onmessage = (ev) => {
        setStoreSubmit("buy_availableBalance", Valut.parse(JSON.parse(ev.data)).balance);
      };

      const sell_availableBalance = new EventSource(
        `${api}/private/balance/sse?${params({
          asset_id: baseAsset.id,
        })}`
      );
      sell_availableBalance.onopen = async () => {
        await fetch(
          `${api}/private/balance?${params({
            asset_id: baseAsset.id,
          })}`
        )
          .then((r) => r.json())
          .then((r) => Valut.parse(r))
          .then((r) => setStoreSubmit("sell_availableBalance", r.balance));
      };
      sell_availableBalance.onmessage = (ev) => {
        setStoreSubmit("sell_availableBalance", Valut.parse(JSON.parse(ev.data)).balance);
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
              availableBalance={storeSubmit.buy_availableBalance}
              quoteAsset={props.quoteAsset}
              baseAsset={props.baseAsset}
              precision={props.precision}
            />
          </div>
          <div class="col-start-2 col-end-3 px-[12px]">
            <SellForm
              availableBalance={storeSubmit.sell_availableBalance}
              quoteAsset={props.quoteAsset}
              baseAsset={props.baseAsset}
              precision={props.precision}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
