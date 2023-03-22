import { JSX } from "solid-js";

export interface CircularButtonComponent {
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  children?: JSX.Element;
  onClick?: (e: MouseEvent) => void;
}

export default function StateActionCircularButton(props: CircularButtonComponent) {
  return (
    <div class={`h-[10px] w-[10px] cursor-pointer select-none rounded-full ${props.class}`} onClick={(e) => props.onClick?.(e)}>
      {props.children}
    </div>
  );
}
