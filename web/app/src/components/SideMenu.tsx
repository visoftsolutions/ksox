import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import SideMenuCircularButton from "./Buttons/SideMenuCircularButton";

export default function SideMenu() {
  return (
    <div class="grid h-full grid-rows-[100px_1fr_100px] justify-center">
      <div class="row-start-1 row-end-2 py-4">
        <img src={joinPaths(base, "/gfx/logo.png")} class="m-auto h-[47px] w-[36px]" />
      </div>
      <div class="row-start-2 row-end-3 ">
        <SideMenuCircularButton class="m-3 bg-gray-3" />
        <SideMenuCircularButton class="m-3 bg-gray-3" />
        <SideMenuCircularButton class="m-3 bg-gray-3" />
      </div>
      <div class="row-start-3 row-end-4 ">
        <SideMenuCircularButton class="m-3 bg-gray-3" />
      </div>
    </div>
  );
}
