import { A } from "solid-start";
import { useNav, Nav } from "~/components/providers/NavProvider";
import SideNavButton from "~/components/SideNav/SideNavButton";
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
    <div class="grid grid-flow-row p-6">
      <div class="grid grid-flow-row">
        <A href="/">
          <SideNavButton
            name="Home"
            highlighted={nav() == Nav.Home}
            icon={
              <Switch>
                <Match when={colorMode.colorMode() == ColorMode.Light}>
                  <img
                    src={joinPaths(base, "/gfx/home-light.svg")}
                    width={24}
                    height={24}
                  ></img>
                </Match>
                <Match when={colorMode.colorMode() == ColorMode.Dark}>
                  <img
                    src={joinPaths(base, "/gfx/home-dark.svg")}
                    width={24}
                    height={24}
                  ></img>
                </Match>
              </Switch>
            }
          />
        </A>
        <A href="/transfer">
          <SideNavButton
            name="Transfer"
            highlighted={nav() == Nav.Transfer}
            icon={
              <Switch>
                <Match when={colorMode.colorMode() == ColorMode.Light}>
                  <img
                    src={joinPaths(base, "/gfx/transfer-light.svg")}
                    width={24}
                    height={24}
                  ></img>
                </Match>
                <Match when={colorMode.colorMode() == ColorMode.Dark}>
                  <img
                    src={joinPaths(base, "/gfx/transfer-dark.svg")}
                    width={24}
                    height={24}
                  ></img>
                </Match>
              </Switch>
            }
          />
        </A>
        <A href="/notifications">
          <SideNavButton
            name="Notifications"
            highlighted={nav() == Nav.Notifications}
            icon={
              <Switch>
                <Match when={colorMode.colorMode() == ColorMode.Light}>
                  <img
                    src={joinPaths(base, "/gfx/bell-light.svg")}
                    width={24}
                    height={24}
                  ></img>
                </Match>
                <Match when={colorMode.colorMode() == ColorMode.Dark}>
                  <img
                    src={joinPaths(base, "/gfx/bell-dark.svg")}
                    width={24}
                    height={24}
                  ></img>
                </Match>
              </Switch>
            }
          />
        </A>
        <A href="/settings">
          <SideNavButton
            name="Settings"
            highlighted={nav() == Nav.Settings}
            icon={
              <Switch>
                <Match when={colorMode.colorMode() == ColorMode.Light}>
                  <img
                    src={joinPaths(base, "/gfx/settings-light.svg")}
                    width={24}
                    height={24}
                  ></img>
                </Match>
                <Match when={colorMode.colorMode() == ColorMode.Dark}>
                  <img
                    src={joinPaths(base, "/gfx/settings-dark.svg")}
                    width={24}
                    height={24}
                  ></img>
                </Match>
              </Switch>
            }
          />
        </A>
      </div>
    </div>
  );
}
