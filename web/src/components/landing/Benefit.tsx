import { JSXElement } from "solid-js";

export default function Benefit(props: { children?: JSXElement }) {
  return (
    <div class="flex items-center gap-[8px]">
      <img width={24} src="/sign.svg" />
      <span>{props.children}</span>
    </div>
  );
}
