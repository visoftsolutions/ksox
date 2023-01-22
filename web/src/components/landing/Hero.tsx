import { A } from "solid-start";
import Benefits from "./Benefits";

export default function Hero() {
  return (
    <div class="flex justify-between">
      <div class="flex flex-col justify-between">
        <span class="[color:#EBEBEB] [font-family:'Lexend'] [font-style:normal] [font-weight:700] [font-size:52px] [line-height:60px]">
          Keep <span class="[color:#00AB82]">Safely</span> On eXchange
        </span>

        <span class="[color:#776A92] [font-family:'Lexend'] [font-style:normal] [font-weight:300] [font-size:20px] [line-height:24px]">
          Powered by STARK a Scalable Transparent Argument of Knowledge for
          computation integrity
        </span>

        <Benefits />

        <div class="flex items-start gap-[24px]">
          <A
            href="/app"
            class="bg-[#5532B9] [border-radius:100px] p-[10px_32px]"
          >
            <span class="[color:#EBEBEB] [font-family:'Lexend'] [font-style:normal] [font-weight:500] [font-size:20px] [line-height:25px]">
              Launch App
            </span>
          </A>

          <A
            href="/learn-more"
            class="[border:1.5px_solid_#EBEBEB] [border-radius:100px] p-[10px_32px]"
          >
            <span class="[color:#EBEBEB] [font-family:'Lexend'] [font-style:normal] [font-weight:500] [font-size:20px] [line-height:25px]">
              Learn more
            </span>
          </A>
        </div>
      </div>
      <img class="w-[590px] h-[376px]" src="/laptop.png" />
    </div>
  );
}
