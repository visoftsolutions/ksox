import Benefits from "./Benefits";
import BigText from "./BigText";
import Buttons from "./Buttons";
import SmallText from "./SmallText";
import Spacing from "~/components/Spacing";
import { base } from "~/root";
import { joinPaths } from "solid-start/islands/server-router";

export default function Hero() {
  return (
    <div class="grid grid-cols-[0.8fr_1fr] gap-8 items-center text-white md:flex-row-reverse">
      <div class="flex flex-col">
        <BigText />
        <Spacing class="h-8" />
        <SmallText />
        <Spacing class="h-8" />
        <Benefits />
        <Spacing class="h-12" />
        <Buttons />
      </div>

      <div>
        <img src={joinPaths(base, "/gfx/laptop.png")} class="mx-auto" />
      </div>
    </div>
  );
}
