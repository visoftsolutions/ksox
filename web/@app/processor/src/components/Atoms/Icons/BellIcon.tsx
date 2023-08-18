import { JSX } from "solid-js/jsx-runtime";
import { SVGComponent } from "./SVGComponent";
import { IIcon } from "./IIcon";

export default function BellIcon(props: IIcon): SVGComponent {
  return (svgProps) => (
    <svg
      width={svgProps?.size || "32px"}
      height={svgProps?.size || "32px"}
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      class={props.className || ""}
      {...svgProps}
    >
      <path
        d="M10.1 5.37363C10.3629 4.57586 11.1142 4 12 4C12.8858 4 13.6371 4.57586 13.9 5.37363C15.7191 6.12152 17 7.91118 17 10V14L19.1464 16.1464C19.4614 16.4614 19.2383 17 18.7928 17H5.20706C4.76161 17 4.53852 16.4614 4.8535 16.1464L7 14V10C7 7.91118 8.28088 6.12152 10.1 5.37363Z"
        fill={svgProps?.stroke || "#FFFFFF"}
      />
      <path
        d="M10 18C10 19.1046 10.8954 20 12 20C13.1046 20 14 19.1046 14 18H10Z"
        fill={svgProps?.stroke || "#FFFFFF"}
      />
    </svg>
  );
}
