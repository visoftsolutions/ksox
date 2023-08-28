import { Match, Switch } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import {
  ColorMode,
  useColorMode,
} from "@packages/components/providers/ColorModeProvider";

export default function ColorModeButton(props: {class: string}) {
  const colorMode = useColorMode();

  return (
    <button
      class={`cursor-pointer border-r-light-text dark:border-r-dark-text border rounded-xl grid items-center justify-center justify-items-center font-bold text-sm p-2 ${props.class}`}
      onClick={() => {
        const values = Object.values(ColorMode);
        colorMode.setColorMode(
          values[(values.indexOf(colorMode.colorMode()) + 1) % values.length],
        );
      }}
    >
      <Switch>
        <Match when={colorMode.colorMode() == ColorMode.Light}>
          <img
            src={joinPaths(base, "/gfx/mode-light.svg")}
            width={20}
            height={20}
          />
        </Match>
        <Match when={colorMode.colorMode() == ColorMode.Dark}>
          <img
            src={joinPaths(base, "/gfx/mode-dark.svg")}
            width={20}
            height={20}
          />
        </Match>
      </Switch>
    </button>
  );
}
