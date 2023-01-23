import { A } from "solid-start";
import Benefits from "./Benefits";

export default function Hero() {
  return (
    <div class="flex justify-between">
      <div class="flex flex-col justify-between text-text-white">
        <div class="text-hero-big font-bold">
          <p>
            Keep <span class="text-secondary">Safely</span> On
          </p>
          <p>eXchange</p>
        </div>

        <span class="text-hero-small font-light text-text-light">
          Powered by STARK a Scalable Transparent Argument of Knowledge for
          computation integrity
        </span>

        <Benefits />

        <div class="flex items-start gap-[24px] text-hero-button font-medium">
          <A
            href="/app"
            class="bg-primary p-[10px_32px] transition-colors [border-radius:100px] hover:bg-text-white hover:text-primary"
          >
            <span>Launch App</span>
          </A>

          <A
            href="#section-1"
            class="p-[10px_32px] transition-colors [border-radius:100px] [border:1.5px_solid_#EBEBEB] hover:bg-text-white hover:text-primary"
          >
            <span>Learn more</span>
          </A>
        </div>
      </div>
      <img class="h-[376px] w-[590px]" src="/laptop.png" />
    </div>
  );
}
