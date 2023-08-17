import { JSX } from "solid-js";
import { SVGComponent } from "./SVGComponent";


export default function Icon(props: {icon: SVGComponent}): JSX.Element {
    return (
        props.icon()
    )
}