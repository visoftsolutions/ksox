import { JSXElement } from "solid-js";

export default function Benefit(props: { children?: JSXElement }) {
  return (
    <div class="flex items-center gap-[8px]">
      <img width={24} src="/sign.svg" />
      <span class="text-text-white [font-family:'Lexend'] [font-style:normal] [font-weight:400] [font-size:15px] [line-height:24px]">
        {props.children}
      </span>
    </div>
  );
}
