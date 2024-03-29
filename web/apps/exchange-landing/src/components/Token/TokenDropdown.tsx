import { For, Show, createSignal } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import { useWallet } from "@packages/components/providers/WalletProvider";
import {
  CONTRACT_ON_CHAIN,
  setCrowdsale,
  useCrowdsale,
} from "~/components/providers/CrowdsaleProvider";

interface TokenDropdownProps {
  disabled: boolean;
}

export default function TokenDropdown(props: TokenDropdownProps) {
  const wallet = useWallet();
  const crowdsale = useCrowdsale();
  const [showDropdown, setShowDropdown] = createSignal<boolean>(false);

  return (
    <button
      class={`relative ${props.disabled ? "cursor-default" : "cursor-pointer"}`}
      onFocusOut={() => {
        setShowDropdown(false);
      }}
    >
      <div
        class="grid grid-cols-[auto_50px_auto] items-center justify-center gap-2 rounded-lg border-[1px] border-slate-600 p-2"
        onClick={() => {
          if (!props.disabled) {
            setShowDropdown(!showDropdown());
          }
        }}
      >
        <div>
          <img
            src={joinPaths(base, crowdsale.selected_token.icon)}
            alt="network"
            width="25px"
            elementtiming={""}
            fetchpriority={"high"}
            class={props.disabled ? "grayscale" : ""}
          />
        </div>
        <div>{crowdsale.selected_token.symbol}</div>
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
        <div class="absolute right-0 top-[50px] z-10 grid gap-2 rounded-lg border border-gray-500 p-2 backdrop-blur-md max-md:w-[200px] md:w-[300px]">
          <div class=" rounded-lg px-4 py-2 font-semibold text-text-1">
            Select Token
          </div>
          <div class="border-[1px] border-solid border-gray-500" />
          <For
            each={
              CONTRACT_ON_CHAIN.get(wallet.selected_network.network.name)
                ?.tokens
            }
          >
            {(item, index) => (
              <div
                data-index={index()}
                class="grid grid-cols-[40px_auto] items-center justify-start gap-2 rounded-lg px-4 py-2 font-semibold text-text-1 transition-colors duration-100 hover:bg-buttonbg_new"
                onClick={() => {
                  setCrowdsale({ selected_token: item });
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
                  <div>{item.name}</div>
                  <div class="text-gray-300">{item.symbol}</div>
                </div>
              </div>
            )}
          </For>
        </div>
      </Show>
    </button>
  );
}
