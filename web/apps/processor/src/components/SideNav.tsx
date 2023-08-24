import { A } from "solid-start";
import { useNav, Nav } from "~/components/providers/NavProvider";
import SideNavButton from "~/components/SideNav/SideNavButton";
import {
  ColorMode,
  useColorMode,
} from "@packages/components/providers/ColorModeProvider";
import { Match, Switch, createMemo } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export default function SideNavigation() {
  const nav = useNav();
  const colorMode = useColorMode();

  const mode = createMemo(() => {
    switch (colorMode.colorMode()) {
      case ColorMode.Light:
        return "Dark Mode";
      case ColorMode.Dark:
        return "Light Mode";
    }
  });

  return (
    <div class="grid grid-flow-row gap-1">
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
                />
              </Match>
              <Match when={colorMode.colorMode() == ColorMode.Dark}>
                <img
                  src={joinPaths(base, "/gfx/home-dark.svg")}
                  width={24}
                  height={24}
                />
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
                />
              </Match>
              <Match when={colorMode.colorMode() == ColorMode.Dark}>
                <img
                  src={joinPaths(base, "/gfx/transfer-dark.svg")}
                  width={24}
                  height={24}
                />
              </Match>
            </Switch>
          }
        />
      </A>
      <SideNavButton
        name={mode()}
        icon={
          <Switch>
            <Match when={colorMode.colorMode() == ColorMode.Light}>
              <img
                src={joinPaths(base, "/gfx/mode-light.svg")}
                width={24}
                height={24}
              />
            </Match>
            <Match when={colorMode.colorMode() == ColorMode.Dark}>
              <img
                src={joinPaths(base, "/gfx/mode-dark.svg")}
                width={24}
                height={24}
              />
            </Match>
          </Switch>
        }
        onClick={() => {
          const values = Object.values(ColorMode);
          colorMode.setColorMode(
            values[(values.indexOf(colorMode.colorMode()) + 1) % values.length],
          );
        }}
      />
    </div>
  );
}
