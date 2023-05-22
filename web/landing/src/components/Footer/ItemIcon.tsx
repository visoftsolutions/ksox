import { JSX } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

interface ItemProps {
    title: string;
    imageSrc: string;
    description?: string;
}

export default function ItemIcon(props: ItemProps) {
    return (
        <div class="grid grid-rows-[auto_auto] gap-1 font-lexend">
            <div class="grid grid-cols-[auto_1fr] justify-center items-center gap-2 row-start-1 row-end-2 font-normal text-md">
                <div class="col-start-1 col-end-2"><img src={joinPaths(base, props.imageSrc)} alt="media" fetchpriority="auto" elementtiming="svg-image" width={25} height={25}/></div>
                <div class="col-start-2 col-end-3 text-ellipsis overflow-hidden">{props.title}</div>
            </div>
            <div class="row-start-2 row-end-3 font-extralight text-sm text-ellipsis overflow-hidden">{props.description}</div>
        </div>
    )
}