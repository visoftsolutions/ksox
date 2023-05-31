import { JSX } from "solid-js";

export interface RectangularButtonComponent {
  highlighted?: boolean;
  disabled?: boolean;
  children?: JSX.Element;
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  onClick?: (e: MouseEvent) => void;
}

export default function NavRectangularButton(props: RectangularButtonComponent) {
  return (
    <div
      class={`grid h-[40px] cursor-pointer select-none items-center justify-center px-3 font-sanspro text-navButton font-semibold
        ${props.class}
        ${props.highlighted ? "bg-ksox-1 text-white" : "text-gray-4"}
        ${props.disabled && "cursor-default text-gray-4"}
      `}
      onClick={(e) => props.onClick?.(e)}
    >
      {props.children}
    </div>
  );
}
