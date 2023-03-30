import { format, parse } from "numerable";
import { createMemo } from "solid-js";
import { createStore } from "solid-js/store";
import { Asset } from "~/types/asset";
import { formatTemplate } from "~/utils/precision";
import SubmitRectangularButton from "../Buttons/SubmitRectangularButton";
import NumberInput from "./NumberInput";
import Slider from "./Slider";

export interface FormComponent {
  quote_asset?: Asset | null;
  base_asset?: Asset | null;
  precision?: number | null;
  available_balance?: number | null;
}

interface SubmitFormComponent {
  price: number;
  quantity: number;
  slider: number;
  value: number;
}

export default function SellForm(props: FormComponent) {
  const [storeComponent, setStoreComponent] = createStore<SubmitFormComponent>({
    price: 0,
    quantity: 0,
    slider: 0,
    value: 0,
  });

  const price = createMemo(() => {
    return format(storeComponent.price, formatTemplate(props.precision ? props.precision : 2));
  });

  const quantity = createMemo(() => {
    return format(storeComponent.quantity, formatTemplate(props.precision ? props.precision : 2));
  });

  const slider = createMemo(() => {
    return props.available_balance && props.available_balance != 0 ? (storeComponent.quantity / props.available_balance) * 100 : 0;
  });

  const value = createMemo(() => {
    return format(storeComponent.value, formatTemplate(props.precision ? props.precision : 2));
  });

  return (
    <div>
      <div class="grid justify-between pb-[4px] text-submit-sublabel font-semibold text-gray-4">
        <div class="col-start-1 col-end-2">Available Balance:</div>
        <div class="col-start-2 col-end-3">
          {format(props.available_balance, formatTemplate(props.precision ? props.precision : 2))} {props.base_asset ? props.base_asset.symbol : "---"}
        </div>
      </div>
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={price()}
        onChange={(e) => {
          const val = parse((e.target as HTMLInputElement).value);
          setStoreComponent("price", val != null && val != 0 ? val : 0);
          setStoreComponent("value", val != null && val != 0 ? storeComponent.quantity * val : 0);
        }}
        precision={props.precision ? props.precision : 2}
        left={"Price"}
        right={props.quote_asset ? props.quote_asset.symbol : "---"}
      />
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={quantity()}
        onChange={(e) => {
          const val = parse((e.target as HTMLInputElement).value);
          setStoreComponent("quantity", val && props.available_balance && val != 0 ? Math.min(props.available_balance, val) : 0);
          setStoreComponent(
            "value",
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
          setStoreComponent("quantity", props.available_balance ? (val / 100) * props.available_balance : 0);
          setStoreComponent("value", props.available_balance && storeComponent.price != 0 ? (val / 100) * props.available_balance * storeComponent.price : 0);
        }}
      />
      <NumberInput
        class="my-[4px] bg-gray-1 p-1 text-submit-label"
        value={value()}
        onChange={(e) => {
          const val = parse((e.target as HTMLInputElement).value);
          setStoreComponent(
            "quantity",
            val && props.available_balance && val != 0 && storeComponent.price != 0
              ? Math.min(props.available_balance * storeComponent.price, val) / storeComponent.price
              : 0
          );
          setStoreComponent("value", val && props.available_balance && val != 0 ? Math.min(props.available_balance * storeComponent.price, val) : 0);
        }}
        precision={props.precision ? props.precision : 2}
        left={"Value"}
        right={props.quote_asset ? props.quote_asset.symbol : "---"}
      />
      <SubmitRectangularButton class="my-[12px] bg-red">Sell</SubmitRectangularButton>
    </div>
  );
}
