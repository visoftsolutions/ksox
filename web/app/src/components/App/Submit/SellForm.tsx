import { format } from "numerable";
import { createStore } from "solid-js/store";
import { fFromWei, fToWeiCeil, fToWeiFloor } from "~/utils/converters/wei";
import { api } from "~/root";
import { SubmitRequest } from "~/types/mod";
import { formatTemplate } from "~/utils/precision";
import SubmitRectangularButton from "./SubmitButton";
import NumberInput from "~/components/Inputs/NumberInput";
import Slider from "~/components/Inputs/Slider";
import { Market } from "~/utils/providers/MarketProvider";
import { Fraction, ev, fFromBigint, finv, fmin, fmul } from "~/types/primitives/fraction";

interface FormValues {
  price: Fraction;
  slider: Fraction;
  base_asset_volume: Fraction;
  quote_asset_volume: Fraction;
}

export default function SellForm(props: { market?: Market; available_balance?: bigint; precision?: number }) {
  const [storeComponent, setStoreComponent] = createStore<FormValues>({
    price: fFromBigint(0n),
    slider: fFromBigint(0n),
    base_asset_volume: fFromBigint(0n),
    quote_asset_volume: fFromBigint(0n),
  });

  return (
    <div>
      <div class="grid justify-between pb-[4px] text-submit-sublabel font-semibold text-gray-4">
        <div class="col-start-1 col-end-2">Available Balance:</div>
        <div class="col-start-2 col-end-3">
          {props.available_balance != undefined ? format(ev(fFromWei(props.available_balance)), formatTemplate(props.precision ?? 2)) : "---"}
          {props.market?.base_asset?.symbol ?? "---"}
        </div>
      </div>
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={storeComponent.price}
        onChange={(price_val) => {
          setStoreComponent("price", price_val);
          setStoreComponent("quote_asset_volume", fmul(storeComponent.base_asset_volume, price_val));
        }}
        precision={props.precision ?? 2}
        left={"Price"}
        right={props.market?.quote_asset?.symbol ?? "---"}
      />
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={storeComponent.base_asset_volume}
        onChange={(base_val) => {
          const max_base_asset_volume = fFromWei(props.available_balance ?? 0n);
          const max_quote_asset_volume = fmul(max_base_asset_volume, storeComponent.price);
          const base_asset_volume = fmin(max_base_asset_volume, base_val);
          setStoreComponent("base_asset_volume", base_asset_volume);
          const quote_asset_volume = fmin(fmul(base_asset_volume, storeComponent.price), max_quote_asset_volume);
          setStoreComponent("quote_asset_volume", quote_asset_volume);
          setStoreComponent("slider", fmul(quote_asset_volume, finv(max_quote_asset_volume)));
        }}
        precision={props.precision ?? 2}
        left={"Quantity"}
        right={props.market?.base_asset?.symbol ?? "---"}
      />
      <Slider
        value={storeComponent.slider}
        inputClass="slider-red"
        onInput={(slider_val) => {
          const max_base_asset_volume = fFromWei(props.available_balance ?? 0n);
          const base_asset_volume = fmul(max_base_asset_volume, slider_val);
          setStoreComponent("base_asset_volume", base_asset_volume);
          const quote_asset_volume = fmul(base_asset_volume, storeComponent.price);
          setStoreComponent("quote_asset_volume", quote_asset_volume);
        }}
      />
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={storeComponent.quote_asset_volume}
        onChange={(quote_val) => {
          const max_base_asset_volume = fFromWei(props.available_balance ?? 0n);
          const max_quote_asset_volume = fmul(max_base_asset_volume, storeComponent.price);
          const quote_asset_volume = fmin(max_quote_asset_volume, quote_val);
          setStoreComponent("quote_asset_volume", quote_asset_volume);
          const base_asset_volume = fmin(fmul(quote_asset_volume, finv(storeComponent.price)), max_base_asset_volume);
          setStoreComponent("base_asset_volume", base_asset_volume);
          setStoreComponent("slider", fmul(quote_asset_volume, finv(max_quote_asset_volume)));
        }}
        precision={props.precision ?? 2}
        left={"Value"}
        right={props.market?.quote_asset?.symbol ?? "---"}
      />
      <SubmitRectangularButton
        class="my-[12px] bg-red"
        onClick={async () => {
          console.log("SELL", storeComponent.base_asset_volume, storeComponent.quote_asset_volume)
          console.log("SELL", fToWeiCeil(storeComponent.base_asset_volume), fToWeiFloor(storeComponent.quote_asset_volume))
          const response = await fetch(`${api}/private/submit`, {
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
                quote_asset_volume: fToWeiCeil(storeComponent.base_asset_volume),
                base_asset_volume: fToWeiFloor(storeComponent.quote_asset_volume),
              }),
              (_, v) => (typeof v === "bigint" ? v.toString() : v)
            ),
          }).then((r) => r.text());
          setStoreComponent("slider", { numerator: 0n, denominator: 1n });
          setStoreComponent("base_asset_volume", { numerator: 0n, denominator: 1n });
          setStoreComponent("quote_asset_volume", { numerator: 0n, denominator: 1n });
        }}
      >
        Sell
      </SubmitRectangularButton>
    </div>
  );
}
