import { Palette } from "../Atoms/Palette";
import { NavButtonProps } from "./NavButton";


export default function SideNavButton(props: NavButtonProps) {
    return (
      <div
        class={`rounded-xl w-48 flex items-center justify-start p-2 ${props.highlighted ? "bg-r-light-foreground dark:bg-r-dark-foreground" : "bg-r-light-background dark:bg-r-dark-background"}`}
      >
        <div class="flex flex-col items-center m-1 mx-3">
          {props.highlighted ? props.icon({stroke: Palette["r-blue"], size: "24px"}) : props.icon({stroke: Palette["r-dark-secondary-text"], size: "24px"})}
        </div>
        <div class={`font-sans text-xs font-bold ${props.highlighted ? "text-r-blue" : "text-r-dark-secondary-text"}`}>
          {props.name}
        </div>
      </div>
    );
  }
  