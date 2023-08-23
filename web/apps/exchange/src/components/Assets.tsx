import { base } from "~/root";
import { joinPaths } from "solid-start/islands/server-router";
import { Index, Match, Switch, createMemo, createSignal } from "solid-js";
import { useAssets } from "@packages/components/providers/AssetsProvider";
import SearchInput from "~/components/Inputs/SearchInput";
import { usePrecision } from "@packages/components/providers/PrecisionProvider";
import { Dynamic } from "solid-js/web";
import CreateAssetInfo from "~/components/Assets/AssetInfo";
import { Asset } from "@packages/types/asset";
import { useSession } from "@packages/components/providers/SessionProvider";
import TriElementHeader from "~/components/App/TriElement/TriElementHeader";
import CreateDeposit from "~/components/Assets/Deposit";
import CreateWithdraw from "~/components/Assets/Withdraw";
import { ContractAddressProvider } from "@packages/components/providers/ContractAddressProvider";

enum Tab {
  Deposit,
  Withdraw,
  History,
  OwnTransfer,
}

export default function Assets() {
  const assets = useAssets();
  const session = useSession();
  const precision = usePrecision();
  const assetsList = createMemo(() => [...assets().values()]);

  const [tab, setTab] = createSignal<Tab>(Tab.Deposit);
  const [selectedAsset, setSelectedAsset] = createSignal<Asset | undefined>(
    undefined,
  );

  return (
    <ContractAddressProvider>
      <div class="col-start-2 col-end-6 row-start-2 row-end-4 grid grid-cols-[320px_200px_1fr] grid-rows-1 gap-[1px] bg-gray-1 font-sanspro">
        <div class="col-start-1 col-end-2 grid h-full grid-rows-[76px_1fr] bg-gray-2">
          <div class="row-start-1 row-end-2 grid items-center px-3">
            <SearchInput
              class="text-markets-searchbar row-start-1 row-end-2 mx-auto mt-3 w-full"
              left={
                <>
                  <img
                    src={joinPaths(base, "/gfx/search.svg")}
                    alt="search"
                    width="20px"
                  />
                </>
              }
            />
            <TriElementHeader
              class="row-start-2 row-end-3 self-end"
              column_0={"Coin"}
            />
          </div>
          <div class="relative row-start-2 row-end-3">
            <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col overflow-y-auto">
              <Index each={assetsList()}>
                {(element, i) => (
                  <div
                    class={`grid h-[56px] cursor-pointer items-center justify-between px-3 py-2 text-orderbook-label
                    ${i % 2 ? "bg-gray-3" : ""}
                    ${
                      element() == selectedAsset()
                        ? "bg-ksox-1 bg-opacity-40 text-white"
                        : ""
                    }`}
                    onClick={() => {
                      setSelectedAsset(element());
                    }}
                  >
                    <div class="col-start-1 col-end-2 inline-grid items-center">
                      <div class="col-start-1 col-end-2 mr-1">
                        <img
                          src={joinPaths(
                            base,
                            "/gfx/asset_icons/" +
                              element().symbol.toLowerCase() +
                              ".svg",
                          )}
                          width="28px"
                          height="28px"
                        />
                      </div>
                      <div class="col-start-2 col-end-3">{`${element().name} (${
                        element().symbol
                      })`}</div>
                    </div>
                  </div>
                )}
              </Index>
            </div>
          </div>
        </div>
        <div class="col-start-2 col-end-3 grid grid-rows-[76px_1fr] bg-gray-2 text-orderbook-label text-gray-4">
          <Dynamic
            component={CreateAssetInfo(session(), selectedAsset(), precision())}
          />
          <div class="row-start-2 row-end-3">
            <div
              class={`mb-[1px] grid h-[36px] cursor-pointer grid-cols-[auto_1fr] items-center justify-center gap-2 px-4 py-2 ${
                tab() == Tab.Deposit ? "bg-ksox-1 bg-opacity-40 text-white" : ""
              } `}
              onClick={() => setTab(Tab.Deposit)}
            >
              <img
                src={joinPaths(base, "/gfx/assets_arrow_down.svg")}
                alt="arrow_down"
                class="col-start-1 col-end-2"
              />
              <div class="col-start-2 col-end-3">Deposit</div>
            </div>
            <div
              class={`mb-[1px] grid h-[36px] cursor-pointer grid-cols-[auto_1fr] items-center justify-center gap-2 px-4 py-2 ${
                tab() == Tab.Withdraw
                  ? "bg-ksox-1 bg-opacity-40 text-white"
                  : ""
              }`}
              onClick={() => setTab(Tab.Withdraw)}
            >
              <img
                src={joinPaths(base, "/gfx/assets_arrow_up.svg")}
                alt="arrow_up"
                class="col-start-1 col-end-2"
              />
              <div class="col-start-2 col-end-3">Withdraw</div>
            </div>
            <div
              class={`mb-[1px] grid h-[36px] cursor-pointer grid-cols-[auto_1fr] items-center justify-center gap-2 px-4 py-2 ${
                tab() == Tab.History ? "bg-ksox-1 bg-opacity-40 text-white" : ""
              }`}
              onClick={() => setTab(Tab.History)}
            >
              <img
                src={joinPaths(base, "/gfx/assets_clock.svg")}
                alt="clock"
                class="col-start-1 col-end-2"
              />
              <div class="col-start-2 col-end-3">History</div>
            </div>
            <div
              class={`mb-[1px] grid h-[36px] cursor-pointer grid-cols-[auto_1fr] items-center justify-center gap-2 px-4 py-2 ${
                tab() == Tab.OwnTransfer
                  ? "bg-ksox-1 bg-opacity-40 text-white"
                  : ""
              }`}
              onClick={() => setTab(Tab.OwnTransfer)}
            >
              <img
                src={joinPaths(base, "/gfx/assets_transfer.svg")}
                alt="transfer"
                class="col-start-1 col-end-2"
              />
              <div class="col-start-2 col-end-3">Own Transfer</div>
            </div>
          </div>
        </div>
        <div class="col-start-3 col-end-4 bg-gray-2 p-4">
          <Switch>
            <Match when={tab() == Tab.Deposit}>
              <Dynamic
                component={CreateDeposit(selectedAsset(), precision())}
              />
            </Match>
            <Match when={tab() == Tab.Withdraw}>
              <Dynamic
                component={CreateWithdraw(selectedAsset(), precision())}
              />
            </Match>
            <Match when={tab() == Tab.History}>
              <div />
            </Match>
            <Match when={tab() == Tab.OwnTransfer}>
              <div />
            </Match>
          </Switch>
        </div>
      </div>
    </ContractAddressProvider>
  );
}
