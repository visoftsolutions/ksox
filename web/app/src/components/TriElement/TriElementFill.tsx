import { JSX } from "solid-js";

export interface TriElementFillComponent {
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  column_0: JSX.Element;
  column_1: JSX.Element;
  column_2: JSX.Element;
  fill: number;
  fill_class?: JSX.HTMLAttributes<HTMLElement>["class"];
}

export default function TriElement(props: TriElementFillComponent) {
  return (
    <div class={`relative grid grid-cols-[0.5fr_1fr] grid-rows-1 items-center justify-center ${props.class}`}>
      <div class="col-start-1 col-end-2 overflow-hidden text-ellipsis text-left font-semibold">{props.column_0}</div>
      <div class="col-start-2 col-end-3">
        <div class="grid grid-cols-2 grid-rows-1 items-center justify-center">
          <div class="col-start-1 col-end-2 overflow-hidden text-ellipsis text-right font-normal">{props.column_1}</div>
          <div class="col-start-2 col-end-3 overflow-hidden text-ellipsis text-right font-normal">{props.column_2}</div>
        </div>
      </div>
      <div
        style={{
          left: `${Math.min(100, Math.max(0, 100 - props.fill * 100)).toString()}%`,
        }}
        class={`absolute top-0 bottom-0 right-0 opacity-20 ${props.fill_class}`}
      />
    </div>
  );
}
