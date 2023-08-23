import { Match, Switch } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import {
  ColorMode,
  useColorMode,
} from "@packages/components/providers/ColorModeProvider";

export interface INotificationsButton {
  highlighted?: boolean;
  class?: string;
}

export default function NotificationsButton(props: INotificationsButton) {
  const colorMode = useColorMode();

  return (
    <button class={`relative h-8 w-8 p-0 ${props.class}`}>
      <Switch>
        <Match when={colorMode.colorMode() == ColorMode.Light}>
          <img src={joinPaths(base, "/gfx/bell-light.svg")} />
        </Match>
        <Match when={colorMode.colorMode() == ColorMode.Dark}>
          <img src={joinPaths(base, "/gfx/bell-dark.svg")} />
        </Match>
      </Switch>
      {props.highlighted ? (
        <div class="z-100 absolute right-1 top-0 h-1 w-1 rounded-full bg-red"></div>
      ) : null}
    </button>
  );
}
