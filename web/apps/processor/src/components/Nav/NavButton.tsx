import { JSX } from "solid-js";

export interface NavButtonProps {
  icon: JSX.Element;
  name: string;
  highlighted?: boolean;
}

export default function NavButton(props: NavButtonProps) {
  return (
    <div class="grid grid-rows-[auto_auto] items-center justify-center p-2 bg-r-light-foreground dark:bg-r-dark-foreground">
      <div class="row-start-1 row-end-2 flex flex-col items-center">
        {props.icon}
      </div>
      <div
        class={`row-start-2 row-end-3 font-sans text-xs font-bold ${
          props.highlighted ? "text-r-blue" : "text-r-dark-secondary-text"
        }`}
      >
        {props.name}
      </div>
    </div>
  );
}
