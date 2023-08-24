import { useSelectedAsset } from "~/components/providers/SelectedAssetProvider";
import { joinPaths } from "solid-start/islands/server-router";
import Picture from "~/components/Atoms/Picture";
import { base } from "~/root";
import ArrowDownButton from "~/components/Atoms/Buttons/ArrowDownButton";
import { Index, Show, createEffect, createMemo, createSignal, onMount } from "solid-js";
import HalfScreenModal from "~/components/Modals/HalfScreenModal";
import Currency from "~/components/Atoms/Currency";
import { useAssets } from "@packages/components/providers/AssetsProvider";

export default function CurrencyDisplay() {
  const selectedAsset = useSelectedAsset();
  const [modal, setModal] = createSignal<boolean>(false);
  const assets = useAssets();
  const assetsList = createMemo(() => [...assets().values()]);
  
  createEffect(() => {
    selectedAsset.setSelectedAsset(assetsList()[0])
  })

  return (
    <div>
      <div class="px-4 pt-4">
        <div class="grid grid-cols-[1fr_auto] justify-between">
          <div class="grid grid-cols-3 grid-rows-2 justify-self-start gap-1">
            <p class="col-start-1 col-end-2 row-start-1 row-end-2  text-bold font-sans text-3xl text-r-light-text dark:text-r-dark-text">
              {selectedAsset.selectedAsset() != undefined ? 0.001 : "---"}
            </p>
            <p class="col-start-2 col-end-3 row-start-1 row-end-2 font-sans text-3xl text-r-dark-secondary-text">
              {selectedAsset.selectedAsset()?.symbol ?? "---"}
            </p>
            <p class="col-start-1 col-end-4 row-start-2 row-end-3 text-sans text-sm text-r-dark-secondary-text">
              {selectedAsset.selectedAsset()?.name ?? "---"}
            </p>
            <div class="col-start-3 col-end-4 row-start-1 row-end-2 self-center justify-self-start">
              <ArrowDownButton class="" onClick={() => setModal(true)} />
            </div>
          </div>
          <Picture
            src={joinPaths(
              base,
              "/gfx/asset_icons/" +
                selectedAsset.selectedAsset()?.symbol.toLowerCase() +
                ".svg",
            )}
            alt="test"
            skeleton={selectedAsset.selectedAsset() == undefined}
          />
        </div>
      </div>
      <Show when={modal()}>
        <HalfScreenModal close={() => setModal(false)}>
          <Index each={assetsList()}>
            {(element) => (
              <Currency
                asset={element()}
                selected={selectedAsset.selectedAsset() == element()}
                onClick={() => {
                  selectedAsset.setSelectedAsset(element());
                }}
              />
            )}
          </Index>
        </HalfScreenModal>
      </Show>
    </div>
  );
}
