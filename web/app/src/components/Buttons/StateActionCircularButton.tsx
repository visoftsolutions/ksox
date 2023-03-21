import { JSX } from "solid-js";

export interface CircularButtonProps {
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  children?: JSX.Element;
  onClick?: (e: MouseEvent) => void;
}

export default function StateActionCircularButton(props: CircularButtonProps) {
  return (
    <div class={`h-[10px] w-[10px] cursor-pointer rounded-full ${props.class}`} onClick={(e) => props.onClick?.(e)}>
      {props.children}
    </div>
  );
}
