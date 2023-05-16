import { For, Show, createSignal } from "solid-js";
import { unwrap } from "solid-js/store";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import { useWallet } from "~/utils/providers/WalletProvider";
import { AVAILABLE_CHAINS } from "../contracts/chains";

interface NetworkDropdownProps {
  disabled: boolean;
}

export default function NetworkDropdown(props: NetworkDropdownProps) {
  const wallet = useWallet();
  const [showDropdown, setShowDropdown] = createSignal<boolean>(false);

  return (
    <button
      class={`relative ${props.disabled ? "cursor-default" : "cursor-pointer"}`}
      onFocusOut={() => {
        setShowDropdown(false);
      }}
    >
      <div
        class="grid grid-cols-2 items-center justify-center gap-2 rounded-lg border-[1px] border-slate-600 p-2"
        onClick={() => {
          if (!props.disabled) {
            setShowDropdown(!showDropdown());
          }
        }}
      >
        <div>
          <img
            src={joinPaths(base, wallet.selected_network?.icon)}
            alt="network"
            width="25px"
            elementtiming={""}
            fetchpriority={"high"}
            class={props.disabled ? "grayscale" : ""}
          />
        </div>
        <div>
          <img
            src={joinPaths(base, "/gfx/down-arrow.svg")}
            alt="arrow"
            width="20px"
            elementtiming={""}
            fetchpriority={"high"}
            class={props.disabled ? "grayscale" : ""}
          />
        </div>
      </div>
      <Show when={showDropdown()}>
        <div class="absolute right-0 top-[50px] grid w-[300px] grid-flow-row gap-2 rounded-lg border border-gray-500 p-2 backdrop-blur-xl">
          <div class="rounded-lg px-4 py-2 font-semibold text-text-1">
            Select Network
          </div>
          <div class="border-[1px] border-solid border-gray-500" />
          <For each={AVAILABLE_CHAINS}>
            {(item, index) => (
              <div
                data-index={index()}
                class="grid grid-cols-[40px_1fr] items-center gap-2 rounded-lg px-4 py-2 font-semibold text-text-1 transition-colors duration-100 hover:bg-buttonbg_new"
                onClick={async () => {
                  try {
                    await wallet.walletClient?.addChain({
                      chain: unwrap(item.network),
                    });
                  } catch (error) {
                    console.log(error);
                  }

                  try {
                    await wallet.walletClient?.switchChain({
                      id: unwrap(item.network).id,
                    });
                  } catch (error) {
                    console.log(error);
                  }

                  setShowDropdown(false);
                }}
              >
                <div class="col-start-1 col-end-2">
                  <img
                    src={joinPaths(base, item.icon)}
                    width="30px"
                    elementtiming={""}
                    fetchpriority={"high"}
                  />
                </div>
                <div class="col-start-2 col-end-3 text-left">
                  {item.network.name}
                </div>
              </div>
            )}
          </For>
        </div>
      </Show>
    </button>
  );
}
