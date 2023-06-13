import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import ProgressBar from "./ProgressBar";

export interface BadgeProps {
  name: string;
  description: string;
  image?: string;
  progress: number;
}

export default function Badge(props: BadgeProps) {
  return (
    <div class="grid grid-rows-[auto_auto_auto] items-center justify-stretch gap-1 bg-gray-2 pb-2">
      <div class="grid grid-cols-[auto_1fr] items-center justify-center">
        <img
          src={props.image ? joinPaths(base, `gfx/badges/${props.image}`) : joinPaths(base, "gfx/badges/default.svg")}
          alt="badge"
          class="h-16 w-16 rounded-full"
        />
        <div class="p-1 text-xl font-extrabold">{props.name}</div>
      </div>
      <div class="px-2 pb-4">
        <ProgressBar fill={props.progress} disable={false} />
      </div>
      <div class="px-2 text-xs font-light">{props.description}</div>
    </div>
  );
}
