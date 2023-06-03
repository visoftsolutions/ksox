import { onMount } from "solid-js";
import SearchInput from "./Inputs/SearchInput";
import { createStore } from "solid-js/store";
import { api } from "~/root";
import { MeRequest } from "@web/types/mod";
import { useSession } from "@web/components/providers/SessionProvider";

interface UserData {
  name?: string;
  email?: string;
  phone?: string;
}

export default function Account() {
  const [userData, setUserData] = createStore<UserData>({});
  const session = useSession();

  onMount(async () => {
    const response = await fetch(`${api}/private/me`, {
      method: "GET",
      credentials: "same-origin",
    })
      .then((r) => r.json())
      .then((r) => MeRequest.parse(r));

    setUserData({
      name: response.name ?? undefined,
      email: response.email ?? undefined,
      phone: response.phone ?? undefined,
    });
  });

  return (
    <div class="grid grid-flow-row items-center justify-center gap-3 bg-gray-2 p-4">
      <SearchInput left="name" value={userData.name} onInput={(e) => setUserData({ name: e.target.value })} />
      <SearchInput left="email" value={userData.email} onInput={(e) => setUserData({ email: e.target.value })} />
      <SearchInput left="phone" value={userData.phone} onInput={(e) => setUserData({ phone: e.target.value })} />
      <div
        class={`grid h-[32px]
                    ${session() ? "cursor-pointer bg-ksox-2 active:bg-opacity-70" : "bg-gray-3"}
                    select-none items-center justify-center rounded-md  text-markets-label transition-colors duration-75`}
        onClick={async () => {
          if (session()) {
            await fetch(`${api}/private/me`, {
              method: "POST",
              headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
              },
              credentials: "same-origin",
              body: JSON.stringify(
                MeRequest.parse({
                  name: userData.name,
                  email: userData.email,
                  phone: userData.phone,
                }),
                (_, v) => (typeof v === "bigint" ? v.toString() : v)
              ),
            }).then((r) => r.text());
          }
        }}
      >
        Submit
      </div>
    </div>
  );
}
