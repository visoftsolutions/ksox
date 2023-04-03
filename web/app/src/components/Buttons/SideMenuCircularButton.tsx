import { JSX } from "solid-js";

export interface CircularButtonComponent {
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  children?: JSX.Element;
  onClick?: (e: MouseEvent) => void;
}

export default function SideMenuCircularButton(props: CircularButtonComponent) {
  return (
    <div
      class={`grid h-[44px] w-[44px] cursor-pointer select-none items-center justify-center rounded-full ${props.class}`}
      onClick={(e) => props.onClick?.(e)}
    >
      {props.children}
    </div>
  );
}
