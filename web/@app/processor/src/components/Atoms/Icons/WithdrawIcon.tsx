import { JSX } from "solid-js/jsx-runtime";
import { SVGComponent } from "./SVGComponent";
import { IIcon } from "./IIcon";

export default function WithdrawIcon(props: IIcon): SVGComponent {
  return (svgProps) => (
    <svg
      width={props.size || "32px"}
      height={props.size || "32px"}
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      class={props.className || ""}
      {...svgProps}
    >
      <path
        d="M7 10H6.2C5.0799 10 4.51984 10 4.09202 10.218C3.71569 10.4097 3.40973 10.7157 3.21799 11.092C3 11.5198 3 12.0799 3 13.2V16.8C3 17.9201 3 18.4802 3.21799 18.908C3.40973 19.2843 3.71569 19.5903 4.09202 19.782C4.51984 20 5.0799 20 6.2 20H17.8C18.9201 20 19.4802 20 19.908 19.782C20.2843 19.5903 20.5903 19.2843 20.782 18.908C21 18.4802 21 17.9201 21 16.8V13.2C21 12.0799 21 11.5198 20.782 11.092C20.5903 10.7157 20.2843 10.4097 19.908 10.218C19.4802 10 18.9201 10 17.8 10H17M12 4V16M12 4L9 7M12 4L15 7"
        stroke={props.stroke || "#FFFFFF"}
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
    </svg>
  );
}
