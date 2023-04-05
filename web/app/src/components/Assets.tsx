import { base } from "~/root";
import { joinPaths } from "solid-start/islands/server-router";
import { Index, JSX, Match, Show, Switch, createEffect, createSignal } from "solid-js";
import { createStore } from "solid-js/store";
import { useAssets } from "~/utils/providers/AssetsProvider";
import { Uuid } from "~/types/primitives/uuid";
import SearchInput from "./Inputs/SearchInput";
import { useSession } from "~/utils/providers/SessionProvider";
import Mint from "./Assets/Mint";
import Burn from "./Assets/Burn";

export interface AssetInfo {
  id: Uuid;
  icon: JSX.Element;
  name: string;
  symbol: string;
  balance: bigint;
}

enum Tab {
  Mint,
  Burn,
  History,
  OwnTransfer,
}

export default function Assets() {
  const assets = useAssets();
  const [search, setSearch] = createSignal<string>("");
  const [tab, setTab] = createSignal<Tab>(Tab.Mint);
  const [assetsState, setAssetsState] = createStore<{
    assets: AssetInfo[];
    selected_asset?: AssetInfo;
    amount: bigint;
  }>({ assets: [], selected_asset: undefined, amount: 0n });
  const precision = 3;

  createEffect(() => {
    if (assets()) {
      assets().forEach((e) => {
        setAssetsState("assets", (prev) => [
          ...prev,
          {
            id: e.id,
            icon: <></>,
            name: e.name,
            symbol: e.symbol,
            balance: 0n,
          },
        ]);
      });
    }
  });

  return (
    <>
      <div class="col-start-2 col-end-6 row-start-2 row-end-4 grid grid-cols-[320px_200px_1fr] grid-rows-1 gap-[1px] bg-gray-1 font-sanspro">
        <div class="col-start-1 col-end-2 grid grid-rows-[80px_1fr] bg-gray-2">
          <div class="row-start-1 row-end-2 px-4 pt-4">
            <div class="grid h-full grid-rows-[auto_1fr]">
              <SearchInput
                class="row-start-1 row-end-2 mx-auto mb-2 w-full text-markets-searchbar"
                left={<img src={joinPaths(base, "/gfx/search.svg")} />}
                onInput={(e) => {
                  const value = (e.target as HTMLInputElement).value;
                  setSearch(value);
                }}
              />
              <div class="row-start-2 row-end-3 grid items-end justify-between overflow-hidden text-ellipsis text-markets-sublabel font-semibold text-gray-4">
                <div class="col-start-1 col-end-2">Coin</div>
                <div class="col-start-2 col-end-3">Balance</div>
              </div>
            </div>
          </div>
          <div class="relative row-start-2 row-end-3">
            <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col overflow-y-auto">
              <Index each={assetsState.assets}>
                {(element, i) => (
                  <div
                    class={`grid h-[56px] cursor-pointer items-center justify-between px-4 py-2 text-orderbook-label
                    ${i % 2 ? "bg-gray-3" : ""}
                    ${element() == assetsState.selected_asset ? "bg-ksox-1 bg-opacity-40 text-white" : ""}`}
                    onClick={() => {
                      setAssetsState({ selected_asset: element() });
                    }}
                  >
                    <div class="col-start-1 col-end-2 inline-grid items-center">
                      <div class="col-start-1 col-end-2 mr-1">{element().icon}</div>
                      <div class="col-start-2 col-end-3">{`${element().name} (${element().symbol})`}</div>
                    </div>
                    {/* <div class="col-start-2 col-end-3">{format(fromWei(element().balance), formatTemplate(precision))}</div> */}
                  </div>
                )}
              </Index>
            </div>
          </div>
        </div>
        <div class="col-start-2 col-end-3 grid grid-rows-[80px_1fr] bg-gray-2 text-orderbook-label text-gray-4">
          <Show when={assetsState.selected_asset}>
            <div class="row-start-1 row-end-2 grid grid-cols-[80px_1fr] items-center justify-center ">
              <div class="col-start-1 col-end-2 mr-2">{assetsState.selected_asset!.icon}</div>
              <div class="col-start-2 col-end-3 text-white">
                {`${assetsState.selected_asset!.name} (${assetsState.selected_asset!.symbol})`}
                {/* <div class="row-start-1 row-end-2 text-white">{`${assetsState.selected_asset!.name} (${assetsState.selected_asset!.symbol})`}</div> */}
                {/* <div class="row-start-2 row-end-3 text-orderbook-item">{format(fromWei(assetsState.selected_asset!.balance), formatTemplate(precision))}</div> */}
              </div>
            </div>
            <div class="row-start-2 row-end-3">
              <div
                class={`mb-[1px] grid h-[36px] cursor-pointer grid-cols-[auto_1fr] items-center justify-center gap-2 px-4 py-2 ${
                  tab() == Tab.Mint ? "bg-ksox-1 bg-opacity-40 text-white" : ""
                } `}
                onClick={() => setTab(Tab.Mint)}
              >
                <img src={joinPaths(base, "/gfx/assets_arrow_down.svg")} class="col-start-1 col-end-2" />
                <div class="col-start-2 col-end-3">Mint</div>
              </div>
              <div
                class={`mb-[1px] grid h-[36px] cursor-pointer grid-cols-[auto_1fr] items-center justify-center gap-2 px-4 py-2 ${
                  tab() == Tab.Burn ? "bg-ksox-1 bg-opacity-40 text-white" : ""
                }`}
                onClick={() => setTab(Tab.Burn)}
              >
                <img src={joinPaths(base, "/gfx/assets_arrow_up.svg")} class="col-start-1 col-end-2" />
                <div class="col-start-2 col-end-3">Burn</div>
              </div>
              <div
                class={`mb-[1px] grid h-[36px] cursor-pointer grid-cols-[auto_1fr] items-center justify-center gap-2 px-4 py-2 ${
                  tab() == Tab.History ? "bg-ksox-1 bg-opacity-40 text-white" : ""
                }`}
                onClick={() => setTab(Tab.History)}
              >
                <img src={joinPaths(base, "/gfx/assets_clock.svg")} class="col-start-1 col-end-2" />
                <div class="col-start-2 col-end-3">History</div>
              </div>
              <div
                class={`mb-[1px] grid h-[36px] cursor-pointer grid-cols-[auto_1fr] items-center justify-center gap-2 px-4 py-2 ${
                  tab() == Tab.OwnTransfer ? "bg-ksox-1 bg-opacity-40 text-white" : ""
                }`}
                onClick={() => setTab(Tab.OwnTransfer)}
              >
                <img src={joinPaths(base, "/gfx/assets_transfer.svg")} class="col-start-1 col-end-2" />
                <div class="col-start-2 col-end-3">Own Transfer</div>
              </div>
            </div>
          </Show>
        </div>
        <div class="col-start-3 col-end-4 bg-gray-2 p-4">
          <Show when={assetsState.selected_asset}>
            <Switch>
              <Match when={tab() == Tab.Mint}>
                <Mint asset={assetsState.selected_asset!} precision={precision} />
              </Match>
              <Match when={tab() == Tab.Burn}>
                <Burn asset={assetsState.selected_asset!} precision={precision} />
              </Match>
              <Match when={tab() == Tab.History}>
                <div></div>
              </Match>
              <Match when={tab() == Tab.OwnTransfer}>
                <div></div>
              </Match>
            </Switch>
          </Show>
        </div>
      </div>
    </>
  );
}
