import { formatDate } from "@packages/utils/formatDate";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import { formatHumanReadable } from "../Inputs/NumberInput";
import { DisplayTransferDirection } from "@packages/types/transfer";

export interface ITransferElement {
  name: string;
  otherName: string;
  date: Date;
  amount: number;
  symbol: string;
  direction: DisplayTransferDirection;
}

export default function TransferElement(props: ITransferElement) {
  return (
    <div class="rounded-xl cursor-pointer">
      <div class="grid grid-cols-[1fr_auto]">
        <div class="grid grid-rows-2 justify-start items-center">
          <div class="font-sans font-bold grid grid-cols-[auto_auto_auto] items-center gap-2">
            <div>{props.name}</div>
            <img
              src={joinPaths(base, "/gfx/right_arrow.svg")}
              width={24}
              height={24}
            />
            <div>{props.otherName}</div>
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
            {`${
              props.direction == DisplayTransferDirection.Income ? "+" : "-"
            }${formatHumanReadable(props.amount.toString(), 3)}`}
          </p>
          <p class="font-sans text-xs text-r-dark-secondary-text">
            {props.symbol}
          </p>
        </div>
      </div>
    </div>
  );
}
