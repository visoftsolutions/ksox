import { Asset } from "@packages/types/asset";
import { formatDate } from "@packages/utils/formatDate";
import { User } from "@packages/types/user";
import firstLastChars from "@packages/utils/firstLastChars";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export interface ITransferElement {
  from: string;
  to: string;
  date: Date;
  amount: number;
  symbol: string;
}

export default function TransferElement(props: ITransferElement) {
  return (
    <div class="rounded-xl cursor-pointer">
      <div class="grid grid-cols-[1fr_auto]">
        <div class="grid grid-rows-2">
          <div class="text-r-light-text dark:text-r-dark-text font-sans font-bold grid grid-cols-[1fr_auto_1fr] items-center justify-center justify-items-center gap-2">
            <div>{props.from}</div>
            <img src={joinPaths(base, "/gfx/right_arrow.svg")} />
            <div>{props.to}</div>
          </div>
          <p class="font-sans text-xs text-r-dark-secondary-text">
            {`${props.date.getHours().toString().padStart(2, "0")}:${props.date
              .getMinutes()
              .toString()
              .padStart(2, "0")} ${formatDate(props.date)}`}
          </p>
        </div>
        <div class="grid grid-rows-2 justify-items-end items-center">
          <p class="text-r-light-text dark:text-r-dark-text font-sans ">
            {`${props.amount > 0 ? "+" : ""}${props.amount}`}
          </p>
          <p class="font-sans text-xs text-r-dark-secondary-text">
            {props.symbol}
          </p>
        </div>
      </div>
    </div>
  );
}
