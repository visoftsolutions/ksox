import { JSX } from "solid-js";

export interface SubmitRectangularButtonComponent {
  children?: JSX.Element;
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  onClick?: (e: MouseEvent) => void;
}

export default function SubmitRectangularButton(
  props: SubmitRectangularButtonComponent,
) {
  return (
    <div
      class={`grid h-[32px] cursor-pointer select-none items-center justify-center rounded-[4px] text-center text-submit-label font-semibold
      ${props.class}`}
      onClick={(e) => props.onClick?.(e)}
    >
      {props.children}
    </div>
  );
}
