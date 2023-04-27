import Benefits from "~/components/main/Benefits";
import BigText from "~/components/main/BigText";
import Buttons from "~/components/main/Buttons";
import SmallText from "~/components/main/SmallText";
import Spacing from "~/components/Spacing";

export default function Hero() {
  return (
    <div class="flex flex-col items-center text-white md:flex-row-reverse md:items-start">
      <div class="md:w-1/2 ">
        <img src="/gfx/laptop.png" class="mx-auto" alt="laptop" />
      </div>
      <Spacing class="h-8" />

      <div class="flex flex-col md:w-1/2">
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
