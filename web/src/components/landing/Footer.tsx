import { A } from "solid-start";
import { DefaultProps } from "../interfaces";

export default function Footer(props: DefaultProps) {
  return (
    <footer class={`${props.class} flex flex-col items-center`}>
      <span class="text-white [font-family:'Lexend'] [font-style:normal] [font-weight:700] [font-size:36px] [line-height:48px]">
        Contact with us!
      </span>
      <span class="text-text-faded [font-family:'Lexend'] [font-style:normal] [font-weight:300] [font-size:20px] [line-height:32px]">
        to learn more abut the project
      </span>

      <div class="mt-[48px] flex justify-between gap-[72px]">
        <div class="flex flex-col items-center">
          <img class="mb-[12px]" src="/phone-icon.svg" />
          <span class="text-white [font-family:'Lexend'] [font-style:normal] [font-size:20px] [font-weight:500] [line-height:25px]">
            +48 601-356-047
          </span>
          <span class="text-text-faded [font-family:'Lexend'] [font-style:normal] [font-weight:300] [font-size:16px] [line-height:20px]">
            call us maybe
          </span>
        </div>

        <div class="flex flex-col items-center">
          <img class="mb-[12px]" src="/mail-icon.svg" />
          <span class="text-white [font-family:'Lexend'] [font-style:normal] [font-size:20px] [font-weight:500] [line-height:25px]">
            ksox.exchange@proton.me
          </span>
          <span class="text-text-faded [font-family:'Lexend'] [font-style:normal] [font-weight:300] [font-size:16px] [line-height:20px]">
            send us a message
          </span>
        </div>

        <div class="flex flex-col items-center">
          <img class="mb-[12px]" src="/mail-icon.svg" />
          <A
            class="text-white [font-family:'Lexend'] [font-style:normal] [font-size:20px] [font-weight:500] [line-height:25px]"
            href="https://twitter.com/KsoxExchange"
            target="_blank"
          >
            KsoxExchange
          </A>
          <span class="text-text-faded [font-family:'Lexend'] [font-style:normal] [font-weight:300] [font-size:16px] [line-height:20px]">
            stay tuned
          </span>
        </div>
      </div>
    </footer>
  );
}
