import Benefits from "~/components/Landing/Benefits";
import BigText from "~/components/Landing/BigText";
import Buttons from "~/components/Landing/Buttons";
import SmallText from "~/components/Landing/SmallText";
import Spacing from "~/components/Spacing";
import { base } from "~/root";
import { joinPaths } from "solid-start/islands/server-router";

export default function Hero() {
  return (
    <div class="grid grid-cols-[0.8fr_1fr] items-center gap-8 text-white max-md:grid-cols-[1fr]">
      <div class="grid grid-flow-row">
        <BigText />
        <Spacing class="h-8" />
        <SmallText />
        <Spacing class="h-8" />
        <Benefits />
        <Spacing class="h-12" />
        <Buttons />
      </div>

      <div class="max-md:hidden">
        <img
          src={joinPaths(base, "/gfx/laptop.png")}
          class="mx-auto"
          elementtiming={""}
          fetchpriority={"high"}
        />
      </div>
    </div>
  );
}
