import Button from "./Atoms/Buttons/Button";
import ArrowDownIcon from "./Atoms/Icons/ArrowDownIcon";
import { Palette } from "./Atoms/Palette";
import Picture from "./Atoms/Picture";

export interface ICurrencyDisplay {
  amount: number;
  code: string;
  currency: string;
  img: string;
}

export default function CurrencyDisplay(props: ICurrencyDisplay) {
  return (
    <div>
      <div class="px-4 pt-4">
        <div class="flex flex-row justify-between">
          <div>
            <div class="flex flex-row">
              <p class="text-bold font-sans text-3xl text-r-dark-text">
                {props.amount}
              </p>
              <p class="pl-2 font-sans text-3xl text-r-dark-secondary-text">
                {props.code}
              </p>
              <Button icon={ArrowDownIcon({ stroke: Palette["r-blue"], size: "20px" })} color="bg-r-blue-dark-backdrop" width="14" height="14"/>
            </div>
            <p class="text-sans text-sm text-r-dark-secondary-text">
              {props.currency}
            </p>
          </div>
          <Picture src={props.img} alt="test" />
        </div>
      </div>
    </div>
  );
}
