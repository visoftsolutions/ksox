import { Show, onCleanup, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import { joinPaths } from "solid-start/islands/server-router";

import subscribeEvents from "@packages/utils/subscribeEvents";
import { SessionResponse } from "@packages/components/providers/SessionProvider/models";
import params from "@packages/utils/params";
import { BadgeFamily, BadgeValue } from "@packages/types/badge";
import { Fraction, ev, fFromBigint } from "@packages/types/primitives/fraction";

import { api, base } from "~/root";
import ProgressBar from "~/components/App/ProgressBar";

export interface BadgeProps {
  badgeFamily: BadgeFamily;
}

interface BadgeStore {
  name?: string;
  description?: string;
  image?: string;
  progress: Fraction;
}

export default function CreateBadge(
  session?: SessionResponse,
  badgeFamily?: BadgeFamily,
) {
  return () => (
    <Show when={session && badgeFamily}>
      <Badge session={session} badgeFamily={badgeFamily} />
    </Show>
  );
}

export function Badge(props: {
  session?: SessionResponse;
  badgeFamily?: BadgeFamily;
}) {
  const [badge, setBadge] = createStore<BadgeStore>({
    name: undefined,
    description: undefined,
    image: undefined,
    progress: fFromBigint(0n),
  });

  let eventsource: EventSource | undefined;

  onMount(async () => {
    if (props.badgeFamily) {
      const badgeFamily = props.badgeFamily;

      eventsource = await subscribeEvents(
        `${api}/private/engagement/badges/open`,
        params({ badge_family: badgeFamily }),
        params({ badge_family: badgeFamily }),
        (data) => {
          const badge = BadgeValue.parse(data);
          setBadge({
            name: badge.name,
            description: badge.description,
            progress: badge.progress,
            // image: badge.name.replaceAll(" ", "").toLowerCase()
          });
        },
      );
    }
  });

  onCleanup(async () => {
    eventsource?.close();
  });

  return (
    <div class="grid grid-flow-row items-center justify-stretch gap-1 bg-gray-2 pb-2">
      <div class="grid grid-cols-[auto_1fr] items-center justify-center">
        <img
          src={
            badge.image
              ? joinPaths(base, `gfx/badges/${badge.image}.svg`)
              : joinPaths(base, "gfx/badges/default.svg")
          }
          alt="badge"
          title={badge.description}
          class="h-16 w-16 rounded-full"
        />
        <div class="p-1 text-xl font-extrabold">{badge.name}</div>
      </div>
      <div class="px-2 pb-4">
        <ProgressBar fill={ev(badge.progress)} disable={false} />
      </div>
      <div class="px-2 text-xs font-light">{`${badge.progress.numer} / ${badge.progress.denom}`}</div>
      <div class="px-2 text-xs font-light">{badge.description}</div>
    </div>
  );
}
