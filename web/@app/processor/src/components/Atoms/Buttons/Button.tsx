import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import { SVGComponent } from "../Icons/SVGComponent";

export interface IButton {
  text?: string;
  icon?: SVGComponent;
  onClick?: () => void;
  color?: string;
  textColor?: string;
  width?: string;
  height?: string;
}

export default function Button(props: IButton) {

  return (
    <button
      class={`relative rounded-full ${props.width || "w-12"} ${
        props.height || "h-12"
      } m-2 p-0 ${props.color || "bg-r-blue-light-backdrop"} ${
        props.textColor || "text-r-blue"
      } flex items-center justify-center text-xs`}
      onClick={props.onClick || (() => {})}
    >
      {props.icon ? props.icon() : null}
      {props.text || null}
    </button>
  );
}
