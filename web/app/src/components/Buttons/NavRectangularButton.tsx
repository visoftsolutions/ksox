import { JSX } from "solid-js";

export interface RectangularButtonComponent {
  highlighted: boolean;
  disabled?: boolean;
  children?: JSX.Element;
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  onClick?: (e: MouseEvent) => void;
}

export default function NavRectangularButton(props: RectangularButtonComponent) {
  return (
    <div
      class={`cursor-pointer select-none rounded-[4px] py-[4px] px-[8px] font-sanspro font-semibold ${
        props.highlighted ? "bg-ksox-1 text-white" : "text-gray-4"
      } ${props.disabled ? "" : "cursor-default text-gray-4"} ${props.class}`}
      onClick={(e) => props.onClick?.(e)}
    >
      {props.children}
    </div>
  );
}
