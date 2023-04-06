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
      class={`cursor-pointer select-none px-3 h-[40px] text-navButton font-sanspro font-semibold grid justify-center items-center ${props.highlighted ? "bg-ksox-1 text-white" : "text-gray-4"
        } ${props.disabled ? "" : "cursor-default text-gray-4"} ${props.class}`}
      onClick={(e) => props.onClick?.(e)}
    >
      {props.children}
    </div>
  );
}
