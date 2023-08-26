import CurrencyDisplay from "~/components/Home/CurrencyDisplay";
import DepositWithdrawPanel from "~/components/Home/DepositWithdrawPanel";
import { Accessor, Index, createEffect, createSignal } from "solid-js";
import UserElement, { IUserElement } from "../Atoms/UserElement";
import { api } from "~/root";
import params from "@packages/utils/params";
import { User, UserRecognitionResult } from "@packages/types/user";
import { z } from "zod";
import { A } from "@solidjs/router";
import { useSelectedUser } from "../providers/SelectedUserProvider";

export default function UserDasboard(props: { search?: string }) {
  const [users, setUsers] = createSignal<UserRecognitionResult[]>([]);
  const selectedUser = useSelectedUser();

  createEffect(async () => {
    const response = await fetch(
      `${api}/public/users/search_user?${params({
        input: props.search,
      })}`,
      {
        method: "GET",
        credentials: "same-origin",
      },
    )
      .then((r) => r.json())
      .then((r) => z.array(UserRecognitionResult).parse(r));

    setUsers(response);
  });

  return (
    <div class="grid grid-rows-[auto_auto_1fr] h-full gap-4">
      <CurrencyDisplay />
      <p class="text-sans text-sm text-bold text-r-dark-secondary-text p-2">
        Users
      </p>
      <div class="relative">
        <div class="absolute inset-0 overflow-y-auto">
          <div class="grid grid-flow-row gap-4">
            <Index each={users()}>
              {(element) => (
                <A
                  href={`/transfer_to/${element().user.address}`}
                  onClick={() => selectedUser.setSelectedUser(element().user)}
                >
                  <UserElement
                    name={element().user.name ?? "- - -"}
                    address={element().user.address}
                  />
                </A>
              )}
            </Index>
          </div>
        </div>
      </div>
    </div>
  );
}
