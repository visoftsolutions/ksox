import { JSX } from "solid-js";

export interface SideNavButtonProps {
  icon: JSX.Element;
  name: string;
  highlighted?: boolean;
}

export default function SideNavButton(props: SideNavButtonProps) {
  return (
    <div
      class={`rounded-xl grid grid-cols-[3rem_1fr] items-center justify-start p-2 ${
        props.highlighted
          ? "bg-r-light-foreground dark:bg-r-dark-foreground"
          : "bg-r-light-background dark:bg-r-dark-background"
      }`}
    >
      <div class="justify-self-center">{props.icon}</div>
      <div
        class={`font-sans text-sm font-bold ${
          props.highlighted ? "text-r-blue" : "text-r-dark-secondary-text"
        }`}
      >
        {props.name}
      </div>
    </div>
  );
}
