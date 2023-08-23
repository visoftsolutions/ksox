import { Match, Switch } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import {
  ColorMode,
  useColorMode,
} from "@packages/components/providers/ColorModeProvider";

export interface IArrowDownButton {
  class?: string;
  onClick: () => void;
}

export default function ArrowDownButton(props: IArrowDownButton) {
  const colorMode = useColorMode();

  return (
    <div
      class={`rounded-full bg-r-blue-light-backdrop dark:bg-r-blue-dark-backdrop ${props.class}`}
      onClick={() => props.onClick()}
    >
      <Switch>
        <Match when={colorMode.colorMode() == ColorMode.Light}>
          <img
            src={joinPaths(base, "/gfx/arrow_down-light.svg")}
            width={20}
            height={20}
          />
        </Match>
        <Match when={colorMode.colorMode() == ColorMode.Dark}>
          <img
            src={joinPaths(base, "/gfx/arrow_down-dark.svg")}
            width={20}
            height={20}
          />
        </Match>
      </Switch>
    </div>
  );
}
