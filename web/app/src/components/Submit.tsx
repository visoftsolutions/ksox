import { createEffect } from "solid-js";
import { createStore } from "solid-js/store";
import { Asset } from "~/types/asset";
import { PRIVATE_URL } from "~/types/mod";
import { Valut } from "~/types/valut";
import params from "~/utils/params";
import RectangularButton from "./Buttons/NavRectangularButton";
import BuyForm from "./Inputs/BuyForm";
import SellForm from "./Inputs/SellForm";

export default function Submit() {
  const [storeState, setStoreState] = createStore<{ quote_asset: Asset; base_asset: Asset; precission: number }>({
    quote_asset: {
      id: "5864a1b9-4ae1-424f-bdb4-f644cb359463",
      created_at: new Date(),
      name: "bitcoin",
      symbol: "USDC",
      maker_fee: {
        numerator: 10,
        denominator: 10,
      },
      taker_fee: {
        numerator: 10,
        denominator: 10,
      },
    },
    base_asset: {
      id: "7a99f052-d941-4dcc-aabd-6695c24deed5",
      created_at: new Date(),
      name: "ethereum",
      symbol: "ETH",
      maker_fee: {
        numerator: 10,
        denominator: 10,
      },
      taker_fee: {
        numerator: 10,
        denominator: 10,
      },
    },
    precission: 3,
  });

  const [storeSubmit, setStoreSubmit] = createStore<{
    buy_available_balance: number;
    sell_available_balance: number;
  }>({
    buy_available_balance: 1323123,
    sell_available_balance: 123124,
  });

  createEffect(async () => {
    const events = await new EventSource(
      `${PRIVATE_URL}/balance/sse?${params({
        asset_id: storeState.quote_asset.id,
      })}`
    );
    events.onopen = async () => {
      const elements = await fetch(
        `${PRIVATE_URL}/balance?${params({
          asset_id: storeState.quote_asset.id,
        })}`
      )
        .then((r) => r.json())
        .then((r) => Valut.parse(r));
      setStoreSubmit("buy_available_balance", (prev) => (prev == undefined ? elements.balance : prev));
    };
    events.onmessage = (msg) => {
      setStoreSubmit("buy_available_balance", Valut.parse(JSON.parse(msg.data)).balance);
    };
  });

  createEffect(async () => {
    const events = await new EventSource(
      `${PRIVATE_URL}/balance/sse?${params({
        asset_id: storeState.base_asset.id,
      })}`
    );
    events.onopen = async () => {
      const elements = await fetch(
        `${PRIVATE_URL}/balance?${params({
          asset_id: storeState.base_asset.id,
        })}`
      )
        .then((r) => r.json())
        .then((r) => Valut.parse(r));
      setStoreSubmit("sell_available_balance", (prev) => (prev == undefined ? elements.balance : prev));
    };
    events.onmessage = (msg) => {
      setStoreSubmit("sell_available_balance", Valut.parse(JSON.parse(msg.data)).balance);
    };
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
              quote_asset={storeState.quote_asset}
              base_asset={storeState.base_asset}
              precission={storeState.precission}
            />
          </div>
          <div class="col-start-2 col-end-3 px-[12px]">
            <SellForm
              available_balance={storeSubmit.sell_available_balance}
              quote_asset={storeState.quote_asset}
              base_asset={storeState.base_asset}
              precission={storeState.precission}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
