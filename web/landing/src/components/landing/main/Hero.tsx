import Benefits from "./Benefits";
import BigText from "./BigText";
import Buttons from "./Buttons";
import SmallText from "./SmallText";
import Spacing from "../../Spacing";

export default function Hero() {
  return (
    <div class="flex flex-col items-center text-white md:flex-row-reverse">
      <div>
        <img src="/gfx/laptop.png" class="mx-auto" />
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
