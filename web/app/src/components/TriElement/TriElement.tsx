import { JSX } from "solid-js";

export interface TriElementDisplay {
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  column_0: JSX.Element;
  column_1: JSX.Element;
  column_2: JSX.Element;
}

export default function TriElement(props: TriElementDisplay) {
  return (
    <div class={`relative grid grid-cols-[0.6fr_1fr] grid-rows-1 items-center justify-center ${props.class}`}>
      <div class="col-start-1 col-end-2 overflow-hidden font-semibold">{props.column_0}</div>
      <div class="col-start-2 col-end-3">
        <div class="grid grid-cols-2 grid-rows-1">
          <div class="col-start-1 col-end-2 overflow-hidden font-normal">{props.column_1}</div>
          <div class="col-start-2 col-end-3 overflow-hidden font-normal">{props.column_2}</div>
        </div>
      </div>
    </div>
  );
}
