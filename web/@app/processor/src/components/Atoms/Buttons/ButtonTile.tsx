import { IButton } from "./Button";

export interface IButtonTile {
  text?: string;
  button: IButton;
}

export default function ButtonTile(props: IButtonTile) {
  return (
    <div class="m-4 flex flex-col items-center">
      <button
        class={`relative rounded-full ${props.button.width || "w-12"} ${
          props.button.height || "h-12"
        } m-2 p-0 ${props.button.color || "bg-r-blue-light-backdrop"} ${props.button.darkColor || "dark:bg-r-blue-dark-backdrop"} ${
          props.button.textColor || "text-r-blue"
        } flex items-center justify-center text-xs`}
        onClick={props.button.onClick || (() => {})}
      >
        {props.button.icon ? props.button.icon() : null}
        {props.button.text || null}
      </button>
      <p class="text-sm text-r-blue font-sans font-medium">{props.text || ""}</p>
    </div>
  );
}
