import { JSX } from "solid-js/jsx-runtime";
import { SVGComponent } from "./SVGComponent";
import { IIcon } from "./IIcon";

export default function ArrowDownIcon(props: IIcon): SVGComponent {
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
      <path
        d="M5.70711 9.71069C5.31658 10.1012 5.31658 10.7344 5.70711 11.1249L10.5993 16.0123C11.3805 16.7927 12.6463 16.7924 13.4271 16.0117L18.3174 11.1213C18.708 10.7308 18.708 10.0976 18.3174 9.70708C17.9269 9.31655 17.2937 9.31655 16.9032 9.70708L12.7176 13.8927C12.3271 14.2833 11.6939 14.2832 11.3034 13.8927L7.12132 9.71069C6.7308 9.32016 6.09763 9.32016 5.70711 9.71069Z"
        fill={props.stroke || "#FFFFFF"}
      />
    </svg>
  );
}
