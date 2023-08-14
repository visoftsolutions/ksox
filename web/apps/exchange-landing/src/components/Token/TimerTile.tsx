export interface TimerTileProps {
  value: string;
  name: string;
  higlighted: boolean;
  disabled: boolean;
}

export default function TimerTile(props: TimerTileProps) {
  return (
    <div
      class={` ${
        props.disabled
          ? "bg-gray-700"
          : props.higlighted
          ? "token-linear-wipe-button"
          : "bg-gray-400"
      } rounded-lg p-[2px]`}
    >
      <div class="grid grid-rows-2 items-center justify-center gap-2 rounded-lg bg-[#00001d] p-4 font-lexend font-bold">
        <div class="row-start-1 row-end-2 overflow-hidden text-ellipsis text-center text-2xl">
          {!props.disabled ? props.value : "X"}
        </div>
        <div class="row-start-2 row-end-3 overflow-hidden text-ellipsis text-center text-lg max-md:text-sm">
          {props.name}
        </div>
      </div>
    </div>
  );
}
