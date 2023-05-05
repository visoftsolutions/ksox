export interface TimerTileProps {
    value: string;
    name: string
}

export default function TimerTile (props: TimerTileProps) {
    return (
        <div class="grid grid-rows-2 p-4 font-lexend text-text-1 font-bold border-slate-600 border-[1px] rounded-lg justify-center items-center gap-2">
            <div class="row-start-1 row-end-2 text-center text-2xl">{props.value}</div>
            <div class="row-start-2 row-end-3 text-center text-lg">{props.name}</div>
        </div>
    );
}