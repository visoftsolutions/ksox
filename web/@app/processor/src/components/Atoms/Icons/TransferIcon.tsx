import { SVGComponent } from "./SVGComponent";
import { IIcon } from "./IIcon";

export default function TransferIcon(props: IIcon): SVGComponent {
  console.log("TransferIcon", props);
  return (svgProps) => (
    <svg
      width={props.size || "32px"}
      height={props.size || "32px"}
      viewBox="0 0 24 24"
      version="1.1"
      xmlns="http://www.w3.org/2000/svg"
      {...svgProps}
      class={props.className || ""}
    >
      <g
        id="Page-1"
        stroke="none"
        stroke-width="1"
        fill="none"
        fill-rule="evenodd"
      >
        <g id="Transfer">
          <rect
            id="Rectangle"
            x="0"
            y="0"
            width="24"
            height="24"
          ></rect>
          <path
            d="M19,7 L5,7 M20,17 L5,17"
            id="Shape"
            stroke-width="1.2"
            stroke-linecap="round"
            stroke={props.stroke || "#FFFFFF"}
          ></path>
          <path
            d="M16,3 L19.2929,6.29289 C19.6834,6.68342 19.6834,7.31658 19.2929,7.70711 L16,11"
            id="Path"
            stroke-width="1.2"
            stroke-linecap="round"
            stroke={props.stroke || "#FFFFFF"}
          ></path>
          <path
            d="M8,13 L4.70711,16.2929 C4.31658,16.6834 4.31658,17.3166 4.70711,17.7071 L8,21"
            id="Path"
            stroke-width="1.2"
            stroke-linecap="round"
            stroke={props.stroke || "#FFFFFF"}
          ></path>
        </g>
      </g>
    </svg>
  );
}