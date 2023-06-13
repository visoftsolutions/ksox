import copyToClipboard from "@web/utils/copyToClipboard";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import { Index } from "solid-js";

export interface BadgesElements {
  name: string;
}

export interface ProfileProps {
  class?: string;
  name: string;
  publicWallet: string;
  image?: string; // if not given then apply default image
  badges?: BadgesElements[];
}

export default function Profile(props: ProfileProps) {
  return (
    <div class="grid grid-cols-[auto_1fr] items-center justify-center gap-6 bg-gray-2 p-2">
      <img
        src={props.image ? joinPaths(base, props.image) : joinPaths(base, "gfx/user.svg")}
        alt="profile photo"
        class="m-2 h-16 w-16 rounded-full md:m-4 md:h-28 md:w-28"
      />
      <div class="grid grid-rows-[auto_auto_auto] gap-1">
        <p class="text-xl font-extrabold md:text-3xl">{props.name}</p>
        <div class="grid grid-cols-[minmax(0px,auto)_auto] items-center justify-start gap-3">
          <p class="overflow-clip text-ellipsis text-base md:text-xl">{props.publicWallet}</p>
          <button onClick={() => copyToClipboard(props.publicWallet)}>
            <img src={joinPaths(base, "gfx/copy.svg")} alt="copy" class="h-6 w-6" />
          </button>
        </div>
        {/* display badges */}
        <div class="flex flex-row flex-wrap gap-1">
          <Index each={props.badges}>
            {(element) => (
              <div>
                <img src={`/gfx/badges/${element().name.replaceAll(" ", "").toLowerCase()}.svg`} alt={element().name} class="h-6 w-6" />
              </div>
            )}
          </Index>
        </div>
      </div>
    </div>
  );
}
