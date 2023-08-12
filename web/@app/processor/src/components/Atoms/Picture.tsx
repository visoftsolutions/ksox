import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export interface IPicture {
    src: string;
    alt?: string;
    size?: number;
    className?: string;
}

export default function Picture(props: IPicture) {
    return (
        <div class={`rounded-full ${props.className}`}>
            <img src={joinPaths(base, props.src)} alt={props.alt || ""} style = {{width: `${props.size || 42}px`, height: `${props.size || 42}px`}}/>
        </div>
    )
}