import { DefaultProps } from "~/utils/interfaces";
import { base } from "~/root";
import { joinPaths } from "solid-start/islands/server-router";

export default function Benefit(props: DefaultProps) {
  return (
    <div class={`grid grid-cols-[auto_1fr] gap-4 items-center justify-start ${props.class}`}>
      <div>
        <img class="w-6" src={joinPaths(base, "/gfx/sign.svg")} />
      </div>

      <p class="text-left font-lexend text-hero-benefit font-normal text-text-1">
        {props.children}
      </p>
    </div>
  );
}
