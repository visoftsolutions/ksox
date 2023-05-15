import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import SideMenuCircularButton from "./SideMenu/Buttons/CircularButton";
import { A } from "solid-start";
import { Nav, useNav } from "~/utils/providers/NavProvider";
import { useMarket } from "~/utils/providers/MarketProvider";

export default function SideMenu() {
  const nav = useNav();
  const market = useMarket();

  return (
    <div class="grid h-full grid-rows-[76px_1fr_76px] items-start justify-center gap-4">
      <div class="row-start-1 row-end-2 py-4">
        <A href="/">
          <div>
            <img src={joinPaths(base, "/gfx/logo.png")} alt="ksox logo" class="m-auto h-[47px] w-[36px]" />
          </div>
        </A>
      </div>
      <div class="row-start-2 row-end-3 grid items-center justify-center gap-4">
        <A href={`${market().base_asset && market().quote_asset ? "/" + market().base_asset?.id + "/" + market().quote_asset?.id : "/"}`}>
          <SideMenuCircularButton class="bg-gray-3" highlighted={nav() == Nav.App}>
            <img src={joinPaths(base, "/gfx/side_menu_spot.svg")} alt="spot" class="h-[36px] w-[36px]" />
          </SideMenuCircularButton>
        </A>
        <A href="/assets">
          <SideMenuCircularButton class="bg-gray-3" highlighted={nav() == Nav.Assets}>
            <img src={joinPaths(base, "/gfx/side_menu_assets.svg")} alt="assets" class="h-[36px] w-[36px]" />
          </SideMenuCircularButton>
        </A>
      </div>
      <div class="row-start-3 row-end-4">
        <SideMenuCircularButton class="m-3 bg-gray-3" />
      </div>
    </div>
  );
}
