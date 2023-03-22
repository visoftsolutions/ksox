import { JSX } from "solid-js";

export interface CircularButtonComponent {
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  children?: JSX.Element;
  onClick?: (e: MouseEvent) => void;
}

export default function SideMenuCircularButton(props: CircularButtonComponent) {
  return (
    <div class={`h-[44px] w-[44px] cursor-pointer select-none rounded-full ${props.class}`} onClick={(e) => props.onClick?.(e)}>
      {props.children}
    </div>
  );
}
