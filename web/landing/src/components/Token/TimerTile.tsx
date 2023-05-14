export interface TimerTileProps {
  value: string;
  name: string;
  disabled: boolean;
}

export default function TimerTile(props: TimerTileProps) {
  return (
    <div class={` ${!props.disabled ? "token-linear-wipe-button" : "bg-gray-700"} p-[1px] rounded-lg`}>
      <div class="grid grid-rows-2 items-center justify-center gap-2 rounded-lg p-4 font-lexend font-bold bg-[#00001d]">
      <div class="row-start-1 row-end-2 overflow-hidden text-ellipsis text-center text-2xl">
        { !props.disabled ? props.value : "X"}
      </div>
      <div class="row-start-2 row-end-3 overflow-hidden text-ellipsis text-center text-lg max-md:text-sm">
        {props.name}
      </div>
    </div>
    </div>
  );
}
