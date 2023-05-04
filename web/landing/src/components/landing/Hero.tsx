import Benefits from "./Benefits";
import BigText from "./BigText";
import Buttons from "./Buttons";
import SmallText from "./SmallText";
import Spacing from "~/components/Spacing";
import { base } from "~/root";
import { joinPaths } from "solid-start/islands/server-router";

export default function Hero() {
  return (
    <div class="flex flex-col items-center text-white md:flex-row-reverse">
      <div>
        <img src={joinPaths(base, "/gfx/laptop.png")} class="mx-auto" />
      </div>
      <Spacing class="h-8" />

      <div class="flex flex-col">
        <BigText />
        <Spacing class="h-8" />
        <SmallText />
        <Spacing class="h-8" />
        <Benefits />
        <Spacing class="h-12" />
        <Buttons />
      </div>
    </div>
  );
}
