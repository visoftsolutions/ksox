import { ethers } from "ethers";
import { format, parse } from "numerable";
import { createMemo } from "solid-js";
import { createStore } from "solid-js/store";
import { fromWei, toWei } from "~/converters/wei";
import { api } from "~/root";
import { SubmitRequest } from "~/types/mod";
import params from "~/utils/params";
import { formatTemplate } from "~/utils/precision";
import SubmitRectangularButton from "../Buttons/SubmitRectangularButton";
import { AssetResponse } from "../Markets";
import NumberInput from "./NumberInput";
import Slider from "./Slider";

export interface FormComponent {
  quote_asset?: AssetResponse;
  base_asset?: AssetResponse;
  precision?: number;
  available_balance?: bigint;
}

interface SubmitFormComponent {
  price: number;
  slider: number;
  base_asset_volume: bigint;
  quote_asset_volume: bigint;
}

export default function BuyForm(props: FormComponent) {
  const [storeComponent, setStoreComponent] = createStore<SubmitFormComponent>({
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
    return props.available_balance && props.available_balance != BigInt(0) ?
      fromWei(storeComponent.quote_asset_volume) / fromWei(props.available_balance) * 100 : 0;
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
          {props.quote_asset ? props.quote_asset.symbol : "---"}
        </div>
      </div>
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={price()}
        onChange={(e) => {
          const val = parse((e.target as HTMLInputElement).value) ?? 0;
          setStoreComponent("price", val && val != 0 ? val : 0);
          setStoreComponent("base_asset_volume", val ? toWei(fromWei(storeComponent.quote_asset_volume) / val) : 0n);
        }}
        precision={props.precision ? props.precision : 2}
        left={"Price"}
        right={props.quote_asset ? props.quote_asset.symbol : "---"}
      />
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={base_asset_volume()}
        onChange={(e) => {
          const val = toWei(parse(((e.target as HTMLInputElement).value) ?? 0) ?? 0);
          const max_base_asset_volume = storeComponent.price ? toWei(fromWei(props.available_balance ?? 0n) / storeComponent.price) : 0n;
          const base_asset_volume = val > max_base_asset_volume ? max_base_asset_volume : val
          setStoreComponent("base_asset_volume", base_asset_volume);
          const quote_asset_volume = toWei(fromWei(base_asset_volume ?? 0) * storeComponent.price);
          setStoreComponent("quote_asset_volume", quote_asset_volume);
        }}
        precision={props.precision ? props.precision : 2}
        left={"Quantity"}
        right={props.base_asset ? props.base_asset.symbol : "---"}
      />
      <Slider
        value={slider()}
        onInput={(e) => {
          const slider_val = (e.target as HTMLInputElement).valueAsNumber / 100;
          const quote_asset_volume = toWei(fromWei(props.available_balance ?? 0n) * slider_val);
          setStoreComponent("quote_asset_volume", quote_asset_volume);
          const base_asset_volume = storeComponent.price ? toWei(fromWei(quote_asset_volume ?? 0) / storeComponent.price) : 0n;
          setStoreComponent("base_asset_volume", base_asset_volume);
        }}
      />
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={quote_asset_volume()}
        onChange={(e) => {
          const val = toWei(parse(((e.target as HTMLInputElement).value) ?? 0) ?? 0);
          const quote_asset_volume = val > (props.available_balance ?? 0n) ? (props.available_balance ?? 0n) : val;
          setStoreComponent("quote_asset_volume", quote_asset_volume);
          const base_asset_volume = storeComponent.price ? toWei(fromWei(quote_asset_volume ?? 0) / storeComponent.price) : 0n;
          setStoreComponent("base_asset_volume", base_asset_volume);
        }}
        precision={props.precision ? props.precision : 2}
        left={"Value"}
        right={props.quote_asset ? props.quote_asset.symbol : "---"}
      />
      <SubmitRectangularButton class="my-[12px] bg-green" onClick={async (e) => {
        const response = await fetch(`${api}/private/submit`, {
          method: "POST",
          headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
          },
          credentials: "same-origin",
          body: JSON.stringify(
            SubmitRequest.parse({
              quote_asset_id: props.quote_asset?.id,
              base_asset_id: props.base_asset?.id,
              quote_asset_volume: storeComponent.quote_asset_volume,
              base_asset_volume: storeComponent.base_asset_volume,
            }), (_, v) => typeof v === 'bigint' ? v.toString() : v
          ),
        })
          .then((r) => r.text())
        setStoreComponent("quote_asset_volume", 0n);
        setStoreComponent("base_asset_volume", 0n);
        console.log(response);
      }}>Buy</SubmitRectangularButton>
    </div>
  );
}
