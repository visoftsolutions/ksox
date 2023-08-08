import copyToClipboard from "@web/utils/copyToClipboard";
import { joinPaths } from "solid-start/islands/server-router";
import { api, base } from "~/root";
import { Index, Show, onCleanup, onMount } from "solid-js";
import { SessionResponse } from "@web/components/providers/SessionProvider/models";
import subscribeEvents from "@web/utils/subscribeEvents";
import params from "@web/utils/params";
import { Badge } from "@web/types/badge";
import { User } from "@web/types/user";
import { createStore } from "solid-js/store";

interface ProfileStore {
  name?: string;
  address?: string;
  badges: Badge[];
}

export default function CreateProfile(session?: SessionResponse) {
  return () => (
    <Show when={session}>
      <Profile />
    </Show>
  );
}

export function Profile() {
  const [profile, setProfile] = createStore<ProfileStore>({
    name: undefined,
    address: undefined,
    badges: [],
  });

  let eventsourceUser: EventSource | undefined;
  let eventsourceBadges: EventSource | undefined;

  onMount(async () => {
    eventsourceUser = await subscribeEvents(
      `${api}/private/user`,
      params({}),
      params({}),
      (data) => {
        const user = User.parse(data);
        setProfile({ name: user.name ?? undefined, address: user.address });
      },
    );

    eventsourceBadges = await subscribeEvents(
      `${api}/private/engagement/badges/received`,
      params({}),
      params({}),
      (data) => {
        setProfile("badges", (prev) => [...prev, ...Badge.array().parse(data)]);
      },
    );
  });

  onCleanup(async () => {
    eventsourceUser?.close();
    eventsourceBadges?.close();
  });

  return (
    <div class="grid grid-cols-[auto_1fr] items-center justify-center gap-6 bg-gray-2 p-2">
      <img
        src={joinPaths(base, "gfx/user.svg")}
        alt="profile photo"
        class="m-2 h-16 w-16 rounded-full md:m-4 md:h-28 md:w-28"
      />
      <div class="grid grid-rows-[auto_auto_auto] gap-1">
        <p class="text-xl font-extrabold md:text-3xl">{profile.name}</p>
        <div class="grid grid-cols-[minmax(0px,auto)_auto] items-center justify-start gap-3">
          <Show when={profile.address != undefined}>
            <p class="overflow-clip text-ellipsis text-base md:text-xl">
              {profile.address}
            </p>
            <button onClick={() => copyToClipboard(profile.address ?? "")}>
              <img
                src={joinPaths(base, "gfx/copy.svg")}
                alt="copy"
                class="h-6 w-6"
              />
            </button>
          </Show>
        </div>
        {/* display badges */}
        <div class="flex flex-row flex-wrap gap-1">
          <Index each={profile.badges}>
            {(element) => (
              <div>
                {/* <img src={`/gfx/badges/${element().name.replaceAll(" ", "").toLowerCase()}.svg`} alt={element().description} class="h-6 w-6" /> */}
                <img
                  src={"/gfx/badges/default.svg"}
                  alt={element().description}
                  title={element().description}
                  class="h-6 w-6"
                />
              </div>
            )}
          </Index>
        </div>
      </div>
    </div>
  );
}
