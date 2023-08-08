import { format } from "numerable";
import { createStore } from "solid-js/store";
import { api } from "~/root";
import { SubmitRequest } from "@web/types/mod";
import { formatTemplate } from "@web/utils/precision";
import SubmitRectangularButton from "./SubmitButton";
import NumberInput from "~/components/Inputs/NumberInput";
import Slider from "~/components/Inputs/Slider";
import { Market } from "~/components/providers/MarketProvider";
import {
  Fraction,
  ev,
  fFromBigint,
  finv,
  fmin,
  fmul,
} from "@web/types/primitives/fraction";
import { createEffect, untrack } from "solid-js";

interface FormValues {
  price: Fraction;
  slider: Fraction;
  base_asset_volume: Fraction;
  quote_asset_volume: Fraction;
}

export default function SellForm(props: {
  market?: Market;
  available_balance?: Fraction;
  precision?: number;
}) {
  const [storeComponent, setStoreComponent] = createStore<FormValues>({
    price: fFromBigint(0n),
    slider: fFromBigint(0n),
    base_asset_volume: fFromBigint(0n),
    quote_asset_volume: fFromBigint(0n),
  });

  createEffect(() => {
    const max_base_asset_volume = props.available_balance ?? {
      numer: 0n,
      denom: 1n,
    };
    const { quote_asset_volume, base_asset_volume } = untrack(() => {
      const base_asset_volume = fmin(
        max_base_asset_volume,
        storeComponent.base_asset_volume,
      );
      return {
        base_asset_volume,
        quote_asset_volume:
          storeComponent.price.numer != 0n
            ? fmul(base_asset_volume, storeComponent.price)
            : { numer: 0n, denom: 1n },
      };
    });
    setStoreComponent("base_asset_volume", base_asset_volume);
    setStoreComponent("quote_asset_volume", quote_asset_volume);
    setStoreComponent(
      "slider",
      fmul(base_asset_volume, finv(max_base_asset_volume)),
    );
  });

  return (
    <div>
      <div class="grid justify-between pb-[4px] text-submit-sublabel font-semibold text-gray-4">
        <div class="col-start-1 col-end-2">Available Balance:</div>
        <div class="col-start-2 col-end-3">
          {props.available_balance != undefined
            ? format(
                ev(props.available_balance),
                formatTemplate(props.precision ?? 2),
              )
            : "---"}
          {props.market?.base_asset?.symbol ?? "---"}
        </div>
      </div>
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={storeComponent.price}
        onChange={(price_val) => {
          setStoreComponent("price", price_val);
          setStoreComponent(
            "quote_asset_volume",
            price_val.numer != 0n
              ? fmul(storeComponent.base_asset_volume, price_val)
              : { numer: 0n, denom: 1n },
          );
        }}
        precision={props.precision ?? 2}
        left={"Price"}
        right={props.market?.quote_asset?.symbol ?? "---"}
      />
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={storeComponent.base_asset_volume}
        onChange={(base_val) => {
          const max_base_asset_volume = props.available_balance ?? {
            numer: 0n,
            denom: 1n,
          };
          const base_asset_volume = fmin(max_base_asset_volume, base_val);
          setStoreComponent("base_asset_volume", base_asset_volume);
          const quote_asset_volume =
            storeComponent.price.numer != 0n
              ? fmul(base_asset_volume, storeComponent.price)
              : { numer: 0n, denom: 1n };
          setStoreComponent("quote_asset_volume", quote_asset_volume);
          setStoreComponent(
            "slider",
            fmul(base_asset_volume, finv(max_base_asset_volume)),
          );
        }}
        precision={props.precision ?? 2}
        left={"Quantity"}
        right={props.market?.base_asset?.symbol ?? "---"}
      />
      <Slider
        value={storeComponent.slider}
        inputClass="slider-red"
        onInput={(slider_val) => {
          const max_base_asset_volume = props.available_balance ?? {
            numer: 0n,
            denom: 1n,
          };
          const base_asset_volume = fmul(max_base_asset_volume, slider_val);
          setStoreComponent("base_asset_volume", base_asset_volume);
          const quote_asset_volume =
            storeComponent.price.numer != 0n
              ? fmul(base_asset_volume, storeComponent.price)
              : { numer: 0n, denom: 1n };
          setStoreComponent("quote_asset_volume", quote_asset_volume);
        }}
      />
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={storeComponent.quote_asset_volume}
        onChange={(quote_val) => {
          const max_base_asset_volume = props.available_balance ?? {
            numer: 0n,
            denom: 1n,
          };
          const max_quote_asset_volume = fmul(
            max_base_asset_volume,
            storeComponent.price,
          );
          const quote_asset_volume = fmin(max_quote_asset_volume, quote_val);
          setStoreComponent("quote_asset_volume", quote_asset_volume);
          const base_asset_volume = fmin(
            fmul(quote_asset_volume, finv(storeComponent.price)),
            max_base_asset_volume,
          );
          setStoreComponent("base_asset_volume", base_asset_volume);
          setStoreComponent(
            "slider",
            fmul(base_asset_volume, finv(max_base_asset_volume)),
          );
        }}
        precision={props.precision ?? 2}
        left={"Value"}
        right={props.market?.quote_asset?.symbol ?? "---"}
      />
      <SubmitRectangularButton
        class="my-[12px] bg-red"
        onClick={() => {
          fetch(`${api}/private/submit`, {
            method: "POST",
            headers: {
              Accept: "application/json",
              "Content-Type": "application/json",
            },
            credentials: "same-origin",
            body: JSON.stringify(
              SubmitRequest.parse({
                quote_asset_id: props.market?.base_asset?.id,
                base_asset_id: props.market?.quote_asset?.id,
                price: finv(storeComponent.price),
                quote_asset_volume: storeComponent.base_asset_volume,
                presentation: true,
              }),
              (_, v) => (typeof v === "bigint" ? v.toString() : v),
            ),
          }).then((r) => r.text());
        }}
      >
        Sell
      </SubmitRectangularButton>
    </div>
  );
}
