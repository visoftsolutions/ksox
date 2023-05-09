import { For, JSX, Show, createSignal } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import {
  setCrowdsale,
  useCrowdsale,
} from "~/utils/providers/CrowdsaleProvider";

export default function NetworkDropdown() {
  const crowdsale = useCrowdsale();
  const [showDropdown, setShowDropdown] = createSignal<boolean>(false);

  return (
    <button
      class="relative"
      onfocusout={() => {
        setShowDropdown(false);
      }}
    >
      <div
        class="grid grid-cols-2 items-center justify-center gap-2 rounded-lg border border-gray-500 p-2"
        onclick={() => setShowDropdown(!showDropdown())}
      >
        <div>
          <img
            src={joinPaths(base, "/gfx/asset_icons/eth.svg")}
            alt="network"
            width="20px"
            elementtiming={""}
            fetchpriority={"high"}
          />
        </div>
        <div>
          <img
            src={joinPaths(base, "/gfx/down-arrow.svg")}
            alt="arrow"
            width="20px"
            elementtiming={""}
            fetchpriority={"high"}
          />
        </div>
      </div>
      <Show when={showDropdown()}>
        <div class="absolute right-0 top-[50px] rounded-lg border border-gray-500 p-2 backdrop-blur-xl">
          <For each={crowdsale.available_networks}>
            {(item, index) => (
              <div
                data-index={index()}
                class="grid grid-cols-[40px_1fr] items-center gap-2 rounded-lg px-4 py-2 font-semibold text-text-1 transition-colors duration-100 hover:bg-buttonbg_new"
                onclick={() => {
                  setCrowdsale({ selected_network: item });
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
                <div class="col-start-2 col-end-3 text-left">{item.name}</div>
              </div>
            )}
          </For>
        </div>
      </Show>
    </button>
  );
}
