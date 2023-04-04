import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import SideMenuCircularButton from "./Buttons/SideMenuCircularButton";
import { A } from "solid-start";
import { Nav, useNav } from "~/utils/providers/NavProvider";

export default function SideMenu() {
  const nav = useNav();

  return (
    <div class="grid h-full grid-rows-[100px_1fr_100px] justify-center">
      <div class="row-start-1 row-end-2 py-4">
        <img src={joinPaths(base, "/gfx/logo.png")} class="m-auto h-[47px] w-[36px]" />
      </div>
      <div class="row-start-2 row-end-3 ">
        <A href="/">
          <SideMenuCircularButton class="m-3 bg-gray-3" highlighted={nav() == Nav.Spot}>
            <img src={joinPaths(base, "/gfx/side_menu_spot.svg")} class="h-[36px] w-[36px]" />
          </SideMenuCircularButton>
        </A>
        <A href="/assets">
          <SideMenuCircularButton class="m-3 bg-gray-3" highlighted={nav() == Nav.Assets}>
            <img src={joinPaths(base, "/gfx/side_menu_assets.svg")} class="h-[36px] w-[36px]" />
          </SideMenuCircularButton>
        </A>
      </div>
      <div class="row-start-3 row-end-4 ">
        <SideMenuCircularButton class="m-3 bg-gray-3" />
      </div>
    </div>
  );
}
