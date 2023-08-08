import { Index, Show, createEffect, createSignal } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { api, base } from "~/root";
import { Asset } from "@web/types/asset";
import { Fraction, fFromBigint } from "@web/types/primitives/fraction";
import params from "@web/utils/params";
import NumberInput from "../Inputs/NumberInput";
import { MintBurnRequest, TransferRequest } from "@web/types/mod";
import SearchInput from "../Inputs/SearchInput";
import { z } from "zod";
import { SessionResponse } from "@web/components/providers/SessionProvider/models";
import { User } from "@web/types/user";

export const UserRecognitionResult = z.object({
  score: z.number(),
  user: User,
});
export type UserRecognitionResult = z.infer<typeof UserRecognitionResult>;

const querySearch = async (input: string) => {
  return await fetch(`${api}/public/users/search_user?${params({ input })}`)
    .then((r) => r.json())
    .then((r) => z.array(UserRecognitionResult).parse(r));
};

export default function CreateActions(
  asset?: Asset,
  session?: SessionResponse,
  precision?: number,
) {
  return () => (
    <Show when={asset && precision}>
      <Actions asset={asset} precision={precision} session={session} />
    </Show>
  );
}

export function Actions(props: {
  session?: SessionResponse;
  asset?: Asset;
  precision?: number;
}) {
  const [amount, setAmount] = createSignal<Fraction>(fFromBigint(0n));
  const [search, setSearch] = createSignal<string>("");
  const [users, setUsers] = createSignal<UserRecognitionResult[]>([]);
  const [selectedUser, setSelectedUser] = createSignal<User | undefined>(
    undefined,
  );

  createEffect(() => {
    if (search()) {
      querySearch(search()).then((r) => setUsers(r));
    }
  });

  return (
    <>
      <div class="grid items-center justify-start gap-4">
        <NumberInput
          class="row-start-1 row-end-2 w-72 bg-gray-1 p-1 text-submit-label"
          precision={props.precision}
          left={"Quantity"}
          right={props.asset?.symbol}
          value={amount()}
          onChange={(f) => {
            setAmount(f);
          }}
        />
        <div class="row-start-2 row-end-3 grid grid-cols-2 items-center justify-center justify-items-center">
          <div
            class={`col-start-1 col-end-2 grid h-[32px] w-[100px]
                    ${
                      props.session
                        ? "cursor-pointer bg-ksox-2 active:bg-opacity-70"
                        : "bg-gray-3"
                    }
                    select-none items-center justify-center rounded-md  text-markets-label transition-colors duration-75`}
            onClick={async () => {
              if (props.session) {
                await fetch(`${api}/private/burn`, {
                  method: "POST",
                  headers: {
                    Accept: "application/json",
                    "Content-Type": "application/json",
                  },
                  credentials: "same-origin",
                  body: JSON.stringify(
                    MintBurnRequest.parse({
                      asset_id: props.asset?.id,
                      amount: amount(),
                    }),
                    (_, v) => (typeof v === "bigint" ? v.toString() : v),
                  ),
                }).then((r) => r.text());
              }
            }}
          >
            Burn
          </div>
          <div
            class={`col-start-2 col-end-3 grid h-[32px] w-[100px] 
                    ${
                      props.session
                        ? "cursor-pointer bg-ksox-2 active:bg-opacity-70"
                        : "bg-gray-3"
                    }
                    select-none items-center justify-center rounded-md  text-markets-label transition-colors duration-75`}
            onClick={async () => {
              if (props.session) {
                await fetch(`${api}/private/mint`, {
                  method: "POST",
                  headers: {
                    Accept: "application/json",
                    "Content-Type": "application/json",
                  },
                  credentials: "same-origin",
                  body: JSON.stringify(
                    MintBurnRequest.parse({
                      asset_id: props.asset?.id,
                      amount: amount(),
                    }),
                    (_, v) => (typeof v === "bigint" ? v.toString() : v),
                  ),
                }).then((r) => r.text());
              }
            }}
          >
            Mint
          </div>
        </div>
        <div
          class={`row-start-3 row-end-4 grid h-[32px] 
                    ${
                      props.session
                        ? "cursor-pointer bg-ksox-2 active:bg-opacity-70"
                        : "bg-gray-3"
                    }
                    select-none items-center justify-center rounded-md  text-markets-label transition-colors duration-75`}
          onClick={async () => {
            if (props.session && selectedUser() != undefined) {
              await fetch(`${api}/private/transfer`, {
                method: "POST",
                headers: {
                  Accept: "application/json",
                  "Content-Type": "application/json",
                },
                credentials: "same-origin",
                body: JSON.stringify(
                  TransferRequest.parse({
                    taker_id: selectedUser()?.id,
                    asset_id: props.asset?.id,
                    amount: amount(),
                  }),
                  (_, v) => (typeof v === "bigint" ? v.toString() : v),
                ),
              }).then((r) => r.text());
            }
          }}
        >
          Transfer
        </div>
        <SearchInput
          class="row-start-4 row-end-5 w-72 bg-gray-1 p-1 text-submit-label"
          value={search()}
          onInput={(f) => {
            setSearch(f.target.value);
          }}
        />
        <div class="relative row-start-5 row-end-6 h-[300px]">
          <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col overflow-y-auto">
            <Index each={users()}>
              {(element) => (
                <div
                  class={`grid grid-cols-[auto_1fr] items-center gap-4 py-2 ${
                    selectedUser()?.id == element().user.id
                      ? "bg-gray-3 text-violet-400"
                      : ""
                  }`}
                  onClick={() => {
                    setSelectedUser(element().user);
                  }}
                >
                  <div class="col-start-1 col-end-2">
                    <img
                      src={joinPaths(base, "gfx/user.svg")}
                      alt="user"
                      width="30px"
                      class="m-auto"
                    />
                  </div>
                  <div class="col-start-2 col-end-3 grid grid-flow-row gap-1 break-all text-xs ">
                    <div class="font-medium">id: {element().user.id}</div>
                    <div class="font-medium">
                      address: {element().user.address}
                    </div>
                  </div>
                </div>
              )}
            </Index>
          </div>
        </div>
      </div>
    </>
  );
}
