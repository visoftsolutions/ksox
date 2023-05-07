export interface TimerTileProps {
  value: string;
  name: string;
}

export default function TimerTile(props: TimerTileProps) {
  return (
    <div class="grid grid-rows-2 items-center justify-center gap-2 rounded-lg border-[1px] border-slate-600 p-4 font-lexend font-bold">
      <div class="row-start-1 row-end-2 overflow-hidden text-ellipsis text-center text-2xl">
        {props.value}
      </div>
      <div class="row-start-2 row-end-3 overflow-hidden text-ellipsis text-center text-lg max-md:text-sm">
        {props.name}
      </div>
    </div>
  );
}
