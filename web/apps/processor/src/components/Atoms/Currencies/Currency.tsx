import Picture from "~/components/Atoms/Picture";
import { Asset } from "@packages/types/asset";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export interface ICurrency {
  asset: Asset;
  selected: boolean;
  onClick?: () => void;
}

export default function Currency(props: ICurrency) {
  return (
    <div
      class={`rounded-xl ${
        props.selected
          ? "bg-r-light-modal-selected dark:bg-r-dark-modal-selected"
          : "bg-r-light-foreground dark:bg-r-dark-modal-foreground"
      } cursor-pointer`}
      onClick={() => {
        if (props.onClick) {
          props.onClick();
        }
      }}
    >
      <div class="flex justify-between">
        <div class="m-4 flex">
          <Picture
            src={joinPaths(
              base,
              "/gfx/asset_icons/" + props.asset.symbol.toLowerCase() + ".svg",
            )}
            alt="test"
            size={42}
          />
          <div class="ml-4">
            <p class="text-r-light-text dark:text-r-dark-text font-sans font-bold">
              {props.asset.name}
            </p>
            <p class="font-sans text-xs text-r-dark-secondary-text">
              {props.asset.symbol}
            </p>
          </div>
        </div>
        <div class="m-4 flex flex-col items-end">
          <p class="text-r-light-text dark:text-r-dark-text font-sans ">
            {0.01}
          </p>
        </div>
      </div>
    </div>
  );
}
