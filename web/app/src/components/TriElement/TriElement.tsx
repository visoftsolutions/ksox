import { JSX } from "solid-js";

export interface TriElementComponent {
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  onClick?: JSX.HTMLAttributes<HTMLElement>["onClick"];
  column0: JSX.Element;
  column1: JSX.Element;
  column2: JSX.Element;
}

export default function TriElement(props: TriElementComponent) {
  return (
    <div class={`relative grid grid-cols-[0.5fr_1fr] grid-rows-1 items-center justify-center ${props.class}`}>
      <div class="col-start-1 col-end-2 overflow-hidden text-ellipsis text-left font-semibold">{props.column0}</div>
      <div class="col-start-2 col-end-3">
        <div class="grid grid-cols-2 grid-rows-1 items-center justify-center">
          <div class="col-start-1 col-end-2 overflow-hidden text-ellipsis text-right font-normal">{props.column1}</div>
          <div class="col-start-2 col-end-3 overflow-hidden text-ellipsis text-right font-normal">{props.column2}</div>
        </div>
      </div>
    </div>
  );
}
