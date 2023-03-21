import RectangularButton from "./Buttons/NavRectangularButton";
import SubmitRectangularButton from "./Buttons/SubmitRectangularButton";
import NumberInput from "./Inputs/NumberInput";
import Slider from "./Inputs/Slider";

export interface SubmitFormDisplay {
  available: string;
  order_price?: number;
  slider_value?: number;
  order_value?: number;
}

export interface SubmitDisplay {
  buy: SubmitFormDisplay;
  sell: SubmitFormDisplay;
}

export default function Submit(props: SubmitDisplay) {
  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr]">
      <div class="row-start-1 row-end-2 px-[4px] pt-[12px]">
        <div class="inline-grid grid-cols-[auto_auto_auto] grid-rows-1 gap-1">
          <RectangularButton class="col-start-1 col-end-2" highlighted={true}>
            <span class="text-openorders-label">Limit</span>
          </RectangularButton>
        </div>
      </div>
      <div class="row-start-2 row-end-3 overflow-auto py-[8px]">
        <div class="grid h-full grid-cols-2 grid-rows-1">
          <div class="col-start-1 col-end-2 px-[12px] ">
            <div class="grid justify-between pb-[4px]">
              <div class="col-start-1 col-end-2 text-submit-sublabel font-semibold text-gray-4">Available Balance:</div>
              <div class="col-start-2 col-end-3 text-submit-sublabel font-semibold text-gray-4">{props.buy.available}</div>
            </div>
            <NumberInput class="my-[4px] bg-gray-1 p-1 text-submit-label" left={"Order Price"} right={"USDT"} />
            <NumberInput class="my-[4px] bg-gray-1 p-1 text-submit-label" left={"Quantity"} right={"BTC"} />
            <Slider value={0} />
            <NumberInput class="my-[4px] bg-gray-1 p-1 text-submit-label" left={"Order Value"} right={"USDT"} />
            <SubmitRectangularButton class="my-[12px] bg-green">Buy</SubmitRectangularButton>
          </div>
          <div class="col-start-2 col-end-3 px-[12px]">
            <div class="grid justify-between">
              <div class="col-start-1 col-end-2 text-submit-sublabel font-semibold text-gray-4">Available Balance:</div>
              <div class="col-start-2 col-end-3 text-submit-sublabel font-semibold text-gray-4">{props.sell.available}</div>
            </div>
            <NumberInput class="my-[4px] bg-gray-1 p-1 text-submit-label" left={"Order Price"} right={"USDT"} />
            <NumberInput class="my-[4px] bg-gray-1 p-1 text-submit-label" left={"Quantity"} right={"BTC"} />
            <Slider class="my-[10px] text-submit-label" value={0} />
            <NumberInput class="my-[4px] bg-gray-1 p-1 text-submit-label" left={"Order Value"} right={"USDT"} />
            <SubmitRectangularButton class="my-[12px] bg-red">Sell</SubmitRectangularButton>
          </div>
        </div>
      </div>
    </div>
  );
}
