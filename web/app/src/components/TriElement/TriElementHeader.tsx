import { JSX } from "solid-js";

export interface TriElementHeaderDisplay {
  columns: [JSX.Element, JSX.Element, JSX.Element];
}

export default function TriElementHeader(props: TriElementHeaderDisplay) {
  return (
    <div class="grid grid-cols-[0.6fr_1fr] grid-rows-1 items-center justify-center">
      <div class="col-start-1 col-end-2 overflow-hidden font-sanspro font-semibold text-gray-4">{props.columns[0]}</div>
      <div class="col-start-2 col-end-3">
        <div class="grid grid-cols-2 grid-rows-1">
          <div class="col-start-1 col-end-2 overflow-hidden font-sanspro font-semibold text-gray-4">{props.columns[1]}</div>
          <div class="col-start-2 col-end-3 overflow-hidden font-sanspro font-semibold text-gray-4">{props.columns[2]}</div>
        </div>
      </div>
    </div>
  );
}
