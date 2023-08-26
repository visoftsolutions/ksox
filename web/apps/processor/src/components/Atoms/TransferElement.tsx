import { formatDate } from "@packages/utils/formatDate";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import { formatHumanReadable } from "../Inputs/NumberInput";
import { DisplayTransferDirection } from "@packages/types/transfer";
import { Match, Switch } from "solid-js";
import Picture from "./Picture";

export interface ITransferElement {
  name: string;
  otherName: string;
  date: Date;
  amount: number;
  symbol: string;
  asset_icon_path: string;
  direction: DisplayTransferDirection;
}

export default function TransferElement(props: ITransferElement) {
  return (
    <div class="rounded-xl cursor-pointer">
      <div class="grid grid-cols-[auto_1fr_auto] items-center gap-3">
        <Picture src={props.asset_icon_path} class="aspect-square" />
        <div class="grid grid-rows-2 justify-start items-center">
          <div class="font-sans font-bold grid grid-cols-[auto_auto_auto] items-center gap-2">
            <div>{props.name}</div>
            <Switch>
              <Match when={props.direction == DisplayTransferDirection.Income}>
                <img
                  src={joinPaths(base, "/gfx/left_arrow.svg")}
                  width={24}
                  height={24}
                />
              </Match>
              <Match when={props.direction == DisplayTransferDirection.Outcome}>
                <img
                  src={joinPaths(base, "/gfx/right_arrow.svg")}
                  width={24}
                  height={24}
                />
              </Match>
            </Switch>
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
