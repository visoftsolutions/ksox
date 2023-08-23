import { A } from "solid-start";
import { useNav, Nav } from "~/components/providers/NavProvider";
import NavButton from "~/components/Nav/NavButton";
import {
  ColorMode,
  useColorMode,
} from "@packages/components/providers/ColorModeProvider";
import { Match, Switch } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export default function SideNavigation() {
  const nav = useNav();
  const colorMode = useColorMode();
  return (
    <div class="grid grid-cols-2">
      <A href="/">
        <NavButton
          name="Home"
          highlighted={nav() == Nav.Home}
          icon={
            <Switch>
              <Match when={colorMode.colorMode() == ColorMode.Light}>
                <img
                  src={joinPaths(base, "/gfx/home-light.svg")}
                  width={30}
                  height={30}
                ></img>
              </Match>
              <Match when={colorMode.colorMode() == ColorMode.Dark}>
                <img
                  src={joinPaths(base, "/gfx/home-dark.svg")}
                  width={30}
                  height={30}
                ></img>
              </Match>
            </Switch>
          }
        />
      </A>
      <A href="/transfer">
        <NavButton
          name="Transfer"
          highlighted={nav() == Nav.Transfer}
          icon={
            <Switch>
              <Match when={colorMode.colorMode() == ColorMode.Light}>
                <img
                  src={joinPaths(base, "/gfx/transfer-light.svg")}
                  width={30}
                  height={30}
                ></img>
              </Match>
              <Match when={colorMode.colorMode() == ColorMode.Dark}>
                <img
                  src={joinPaths(base, "/gfx/transfer-dark.svg")}
                  width={30}
                  height={30}
                ></img>
              </Match>
            </Switch>
          }
        />
      </A>
    </div>
  );
}
