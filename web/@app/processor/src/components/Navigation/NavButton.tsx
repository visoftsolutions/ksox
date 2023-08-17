import { base } from "~/root";
import { joinPaths } from "solid-start/islands/server-router";
import { SVGComponent } from "../Atoms/Icons/SVGComponent";
import { Palette } from "../Atoms/Palette";

interface NavButtonProps {
  icon: SVGComponent;
  name: string;
  highlighted?: boolean;
}

export default function NavButton(props: NavButtonProps) {
  return (
    <div
      class="grid grid-rows-[auto_auto] items-center justify-center p-2 bg-r-light-foreground dark:bg-r-dark-foreground"
    >
      <div class="row-start-1 row-end-2 flex flex-col items-center">
        {props.highlighted ? props.icon({stroke: Palette["r-blue"]}) : props.icon({stroke: Palette["r-dark-secondary-text"]})}
      </div>
      <div class={`row-start-2 row-end-3 font-sans text-xs font-bold ${props.highlighted ? "text-r-blue" : "text-r-dark-secondary-text"}`}>
        {props.name}
      </div>
    </div>
  );
}
