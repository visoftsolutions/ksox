import { createSignal } from "solid-js";
import Picture from "../Picture";

export interface ICurrency {
    name: string;
    symbol: string;
    amount: number;
    img: string;
    selected: boolean;
    onClick?: () => void;
}

export default function Currency(props: ICurrency) {

    const handleClick = () => {
        if (props.onClick) {
          props.onClick(); // Invoke the provided onClick function
        //   setCurrentColor("bg-r-light-foreground dark:bg-r-dark-modal-selected");
        }
      };

    return (<div
        class={`rounded-xl ${props.selected ? "bg-r-light-foreground dark:bg-r-dark-modal-selected" : "bg-r-light-foreground dark:bg-r-dark-modal-foreground"} active:bg-black`}
        onClick={handleClick}
      >
        <div class="flex justify-between">
          <div class="m-4 flex">
            <Picture src={props.img} alt="test" size={42} />
            <div class="ml-4">
              <p class="text-r-light-text dark:text-r-dark-text font-sans font-bold">{props.name}</p>
              <p class="font-sans text-xs text-r-dark-secondary-text">
                {props.symbol}
              </p>
            </div>
          </div>
          <div class="m-4 flex flex-col items-end">
            <p class="text-r-light-text dark:text-r-dark-text font-sans ">
              {props.amount}
            </p>
          </div>
        </div>
      </div>)
}