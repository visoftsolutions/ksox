import { ethers } from "ethers";
import { format, parse } from "numerable";
import { createMemo } from "solid-js";
import { createStore } from "solid-js/store";
import { api } from "~/root";
import { SubmitRequest } from "~/types/mod";
import { formatTemplate } from "~/utils/precision";
import SubmitRectangularButton from "../Buttons/SubmitRectangularButton";
import { AssetResponse } from "../Markets";
import NumberInput from "./NumberInput";
import Slider from "./Slider";

export interface FormComponent {
  quote_asset?: AssetResponse;
  base_asset?: AssetResponse;
  precision?: number;
  available_balance?: number;
}

interface SubmitFormComponent {
  price: number;
  slider: number;
  base_asset_volume: number;
  quote_asset_volume: number;
}

export default function SellForm(props: FormComponent) {
  const [storeComponent, setStoreComponent] = createStore<SubmitFormComponent>({
    price: 0,
    slider: 0,
    base_asset_volume: 0,
    quote_asset_volume: 0,
  });

  const price = createMemo(() => {
    return format(storeComponent.price, formatTemplate(props.precision ? props.precision : 2));
  });

  const base_asset_volume = createMemo(() => {
    return format(storeComponent.base_asset_volume, formatTemplate(props.precision ? props.precision : 2));
  });

  const slider = createMemo(() => {
    return props.available_balance && props.available_balance != 0 ? (storeComponent.base_asset_volume / props.available_balance) * 100 : 0;
  });

  const quote_asset_volume = createMemo(() => {
    return format(storeComponent.quote_asset_volume, formatTemplate(props.precision ? props.precision : 2));
  });

  return (
    <div>
      <div class="grid justify-between pb-[4px] text-submit-sublabel font-semibold text-gray-4">
        <div class="col-start-1 col-end-2">Available Balance:</div>
        <div class="col-start-2 col-end-3">
          {props.available_balance != undefined ? format(props.available_balance, formatTemplate(props.precision ? props.precision : 2)) : "---"}
          {props.base_asset ? props.base_asset.symbol : "---"}
        </div>
      </div>
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={price()}
        onChange={(e) => {
          const val = parse((e.target as HTMLInputElement).value);
          setStoreComponent("price", val != null && val != 0 ? val : 0);
          setStoreComponent("quote_asset_volume", val != null && val != 0 ? storeComponent.base_asset_volume * val : 0);
        }}
        precision={props.precision ? props.precision : 2}
        left={"Price"}
        right={props.quote_asset ? props.quote_asset.symbol : "---"}
      />
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={base_asset_volume()}
        onChange={(e) => {
          const val = parse((e.target as HTMLInputElement).value);
          setStoreComponent("base_asset_volume", val && props.available_balance && val != 0 ? Math.min(props.available_balance, val) : 0);
          setStoreComponent(
            "quote_asset_volume",
            val && props.available_balance && val != 0 && storeComponent.price != 0 ? Math.min(props.available_balance, val) * storeComponent.price : 0
          );
        }}
        precision={props.precision ? props.precision : 2}
        left={"Quantity"}
        right={props.base_asset ? props.base_asset.symbol : "---"}
      />
      <Slider
        value={slider()}
        onInput={(e) => {
          const val = (e.target as HTMLInputElement).valueAsNumber;
          setStoreComponent("base_asset_volume", props.available_balance ? (val / 100) * props.available_balance : 0);
          setStoreComponent("quote_asset_volume", props.available_balance && storeComponent.price != 0 ? (val / 100) * props.available_balance * storeComponent.price : 0);
        }}
      />
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={quote_asset_volume()}
        onChange={(e) => {
          const val = parse((e.target as HTMLInputElement).value);
          setStoreComponent(
            "base_asset_volume",
            val && props.available_balance && val != 0 && storeComponent.price != 0
              ? Math.min(props.available_balance * storeComponent.price, val) / storeComponent.price
              : 0
          );
          setStoreComponent("quote_asset_volume", val && props.available_balance && val != 0 ? Math.min(props.available_balance * storeComponent.price, val) : 0);
        }}
        precision={props.precision ? props.precision : 2}
        left={"Value"}
        right={props.quote_asset ? props.quote_asset.symbol : "---"}
      />
      <SubmitRectangularButton class="my-[12px] bg-red" onClick={async (e) => {
        const response = await fetch(`${api}/private/submit`, {
          method: "POST",
          headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
          },
          credentials: "same-origin",
          body: JSON.stringify(
            SubmitRequest.parse({
              quote_asset_id: props.base_asset?.id,
              base_asset_id: props.quote_asset?.id,
              quote_asset_volume: ethers.utils.parseEther(storeComponent.base_asset_volume.toString()),
              base_asset_volume: ethers.utils.parseEther(storeComponent.quote_asset_volume.toString()),
            }), (_, v) => typeof v === 'bigint' ? v.toString() : v
          ),
        })
          .then((r) => r.text())
        console.log(response);
      }}>Sell</SubmitRectangularButton>
    </div>
  );
}
