interface ItemProps {
    title: string;
    description?: string;
}

export default function Item(props: ItemProps) {
    return (
        <div class="grid grid-rows-[auto_auto] gap-1 font-lexend">
            <div class="row-start-1 row-end-2 font-normal text-md text-ellipsis overflow-hidden">{props.title}</div>
            <div class="row-start-2 row-end-3 font-extralight text-sm text-ellipsis overflow-hidden">{props.description}</div>
        </div>
    )
}