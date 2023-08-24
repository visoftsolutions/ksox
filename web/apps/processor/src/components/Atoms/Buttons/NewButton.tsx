import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export default function NewButton(props: { class?: string }) {
  return (
    <button
      class={`rounded-full justify-self-end grid items-center justify-center justify-items-center bg-r-blue p-1 px-2 ${props.class}`}
    >
      <div class="inline-grid grid-cols-[auto_auto] items-center justify-items-center justify-center">
        <img src={joinPaths(base, "/gfx/plus.svg")} width={20} height={20} />
        <div class="text-white font-bold text-xs">New</div>
      </div>
    </button>
  );
}
