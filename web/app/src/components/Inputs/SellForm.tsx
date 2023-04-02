import { format, parse } from "numerable";
import { createMemo } from "solid-js";
import { createStore } from "solid-js/store";
import { fromWei, toWei } from "~/utils/converters/wei";
import { api } from "~/root";
import { SubmitRequest } from "~/types/mod";
import { formatTemplate } from "~/utils/precision";
import SubmitRectangularButton from "../Buttons/SubmitRectangularButton";
import NumberInput from "./NumberInput";
import Slider from "./Slider";
import { Market } from "~/utils/providers/MarketProvider";
import { multiplyFloat } from "~/utils/multiplyFloat";

interface FormValues {
  price: number;
  slider: number;
  base_asset_volume: bigint;
  quote_asset_volume: bigint;
}

export default function SellForm(props: { market?: Market; available_balance?: bigint; precision?: number }) {
  const [storeComponent, setStoreComponent] = createStore<FormValues>({
    price: 0,
    slider: 0,
    base_asset_volume: BigInt(0),
    quote_asset_volume: BigInt(0),
  });

  const price = createMemo(() => {
    return format(storeComponent.price, formatTemplate(props.precision ? props.precision : 2));
  });

  const base_asset_volume = createMemo(() => {
    return format(fromWei(storeComponent.base_asset_volume), formatTemplate(props.precision ? props.precision : 2));
  });

  const slider = createMemo(() => {
    return props.available_balance && props.available_balance != BigInt(0)
      ? (fromWei(storeComponent.base_asset_volume) / fromWei(props.available_balance)) * 100
      : 0;
  });

  const quote_asset_volume = createMemo(() => {
    return format(fromWei(storeComponent.quote_asset_volume), formatTemplate(props.precision ? props.precision : 2));
  });

  return (
    <div>
      <div class="grid justify-between pb-[4px] text-submit-sublabel font-semibold text-gray-4">
        <div class="col-start-1 col-end-2">Available Balance:</div>
        <div class="col-start-2 col-end-3">
          {props.available_balance != undefined ? format(fromWei(props.available_balance), formatTemplate(props.precision ? props.precision : 2)) : "---"}
          {props.market?.base_asset?.symbol ?? "---"}
        </div>
      </div>
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={price()}
        onChange={(e) => {
          const val = parse((e.target as HTMLInputElement).value) ?? 0;
          setStoreComponent("price", val && val != 0 ? val : 0);
          setStoreComponent("quote_asset_volume", multiplyFloat(storeComponent.base_asset_volume, val));
        }}
        precision={props.precision ? props.precision : 2}
        left={"Price"}
        right={props.market?.quote_asset?.symbol ?? "---"}
      />
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={base_asset_volume()}
        onChange={(e) => {
          const base_val = toWei(parse((e.target as HTMLInputElement).value ?? 0) ?? 0);
          const max_base_asset_volume = props.available_balance ?? 0n;
          const base_asset_volume = base_val > max_base_asset_volume ? max_base_asset_volume : base_val;
          setStoreComponent("base_asset_volume", base_asset_volume);
          const quote_asset_volume = multiplyFloat(base_asset_volume, storeComponent.price);
          setStoreComponent("quote_asset_volume", quote_asset_volume);
        }}
        precision={props.precision ? props.precision : 2}
        left={"Quantity"}
        right={props.market?.base_asset?.symbol ?? "---"}
      />
      <Slider
        value={slider()}
        onInput={(e) => {
          const slider_val = (e.target as HTMLInputElement).valueAsNumber / 100;
          const max_base_asset_volume = props.available_balance ?? 0n;
          const base_asset_volume = multiplyFloat(max_base_asset_volume, slider_val);
          setStoreComponent("base_asset_volume", base_asset_volume);
          const quote_asset_volume = multiplyFloat(base_asset_volume, storeComponent.price);
          setStoreComponent("quote_asset_volume", quote_asset_volume);
        }}
      />
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={quote_asset_volume()}
        onChange={(e) => {
          const quote_val = toWei(parse((e.target as HTMLInputElement).value ?? 0) ?? 0);
          const max_base_asset_volume = props.available_balance ?? 0n;
          const max_quote_asset_volume = multiplyFloat(max_base_asset_volume, storeComponent.price);
          const quote_asset_volume = quote_val > max_quote_asset_volume ? max_quote_asset_volume : quote_val;
          setStoreComponent("quote_asset_volume", quote_asset_volume);
          const base_val = storeComponent.price ? multiplyFloat(quote_asset_volume, 1/storeComponent.price) : 0n;
          const base_asset_volume = base_val > max_base_asset_volume ? max_base_asset_volume : base_val;
          setStoreComponent("base_asset_volume", base_asset_volume);
        }}
        precision={props.precision ? props.precision : 2}
        left={"Value"}
        right={props.market?.quote_asset?.symbol ?? "---"}
      />
      <SubmitRectangularButton
        class="my-[12px] bg-red"
        onClick={async () => {
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
                quote_asset_volume: storeComponent.base_asset_volume,
                base_asset_volume: storeComponent.quote_asset_volume,
              }),
              (_, v) => (typeof v === "bigint" ? v.toString() : v)
            ),
          }).then((r) => r.text());
          setStoreComponent("quote_asset_volume", 0n);
          setStoreComponent("base_asset_volume", 0n);
          console.log(response);
        }}
      >
        Sell
      </SubmitRectangularButton>
    </div>
  );
}
