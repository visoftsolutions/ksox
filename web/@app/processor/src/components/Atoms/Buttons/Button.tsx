import { SVGComponent } from "../Icons/SVGComponent";

export interface IButton {
  text?: string;
  icon?: SVGComponent;
  onClick?: () => void;
  color?: string;
  darkColor?: string;
  textColor?: string;
  width?: string;
  height?: string;
  textClass?: string;
  buttonClass?: string;
}

export default function Button(props: IButton) {
  return (
    <button
      class={`relative rounded-full ${props.width || "w-12"} ${
        props.height || "h-12"
      } m-0 p-0 ${props.color ? props.color : "bg-r-blue-light-backdrop"} ${
        props.darkColor ? props.darkColor : ""
      } ${
        props.textColor ? `text-${props.textColor}` : "text-r-blue"
      } flex items-center justify-center text-xs ${props.buttonClass}`}
      onClick={props.onClick || (() => {})}
    >
      {props.icon ? props.icon() : null}
      <p class={`${props.textClass || ""} text-${props.textColor}`}>
        {props.text || null}
      </p>
    </button>
  );
}
