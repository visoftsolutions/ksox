import { A } from "solid-start";
import Benefits from "./Benefits";

export default function Hero() {
  return (
    <div class="flex justify-between">
      <div class="flex flex-col justify-between">
        <span class="text-text-white [font-family:'Lexend'] [font-style:normal] [font-weight:700] [font-size:52px] [line-height:60px]">
          Keep <span class="text-secondary">Safely</span> On eXchange
        </span>

        <span class="text-text-faded [font-family:'Lexend'] [font-style:normal] [line-height:24px] [font-weight:300] [font-size:20px]">
          Powered by STARK a Scalable Transparent Argument of Knowledge for
          computation integrity
        </span>

        <Benefits />

        <div class="flex items-start gap-[24px]">
          <A
            href="/app"
            class="bg-primary p-[10px_32px] text-text-white transition-colors [border-radius:100px] hover:bg-text-white hover:text-primary"
          >
            <span class="[font-family:'Lexend'] [font-style:normal] [font-size:20px] [font-weight:500] [line-height:25px]">
              Launch App
            </span>
          </A>

          <A
            href="#section-1"
            class="p-[10px_32px] text-text-white transition-colors [border-radius:100px] [border:1.5px_solid_#EBEBEB] hover:bg-text-white hover:text-primary"
          >
            <span class="[line-height:25px]hover:bg-[#EBEBEB] [font-family:'Lexend'] [font-style:normal] [font-size:20px] [font-weight:500]">
              Learn more
            </span>
          </A>
        </div>
      </div>
      <img class="h-[376px] w-[590px]" src="/laptop.png" />
    </div>
  );
}
