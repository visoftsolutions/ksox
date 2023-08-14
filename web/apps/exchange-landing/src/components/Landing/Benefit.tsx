import { base } from "~/root";
import { joinPaths } from "solid-start/islands/server-router";
import { JSX } from "solid-js";

interface DefaultProps {
  id?: string;
  style?: JSX.CSSProperties;
  class?: string;
  children?: JSX.Element;
}

export default function Benefit(props: DefaultProps) {
  return (
    <div
      class={`grid grid-cols-[auto_1fr] items-center justify-start gap-4 ${props.class}`}
    >
      <div>
        <img
          class="w-6"
          src={joinPaths(base, "/gfx/sign.svg")}
          elementtiming={""}
          fetchpriority={"high"}
        />
      </div>

      <p class="text-left font-lexend text-hero-benefit font-normal text-text-1">
        {props.children}
      </p>
    </div>
  );
}
