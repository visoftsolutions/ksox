import { JSX } from "solid-js/jsx-runtime";
import { SVGComponent } from "./SVGComponent";
import { IIcon } from "./IIcon";

export default function PlusIcon(props: IIcon): SVGComponent {
  return (svgProps) => (
    <svg
      width={props.size || "32px"}
      height={props.size || "32px"}
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      class={props.className || ""}
      {...svgProps}
      stroke-width="2.4"
    >
      <path d="M6 12H18M12 6V18" stroke={props.stroke || "#FFFFFF"} stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  );
}
