import { JSXElement } from "solid-js";
import { A } from "solid-start";
import { DefaultProps } from "../interfaces";

export default function Section(
  props: DefaultProps & {
    imagePath: string;
    imageClass?: string;
    text0: string;
    text1: string;
  }
) {
  return (
    <div class={`${props.class} flex justify-between gap-[32px]`}>
      <img class={props.imageClass} src={props.imagePath} />
      <div class="flex flex-col items-start gap-[27px]">
        <span class="[color:#00AB82] [font-family:'Lexend'] [font-style:normal] [font-weight:700] [font-size:20px] [line-height:32px]">
          {props.text0}
        </span>

        <span class="[color:#EBEBEB] [font-family:'Lexend'] [font-style:normal] [font-weight:700] [font-size:36px] [line-height:48px]">
          {props.text1}
        </span>

        <span class="[color:#776A92] [font-family:'Lexend'] [font-style:normal] [font-weight:300] [font-size:20px] [line-height:32px]">
          {props.children}
        </span>

        <A
          href="/learn-more"
          class="text-text-faded [font-family:'Lexend'] [font-style:normal] [font-weight:500] [font-size:16px] [line-height:20px] transition-colors hover:text-text-white"
        >
          Learn more
        </A>
      </div>
    </div>
  );
}
