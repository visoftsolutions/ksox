import { useSelectedAsset } from "~/components/providers/SelectedAssetProvider";
import { joinPaths } from "solid-start/islands/server-router";
import Picture from "~/components/Atoms/Picture";
import { base } from "~/root";
import ArrowDownButton from "~/components/Atoms/Buttons/ArrowDownButton";
import { Index, Show, createMemo, createSignal } from "solid-js";
import HalfScreenModal from "~/components/Modals/HalfScreenModal";
import Currency from "~/components/Atoms/Currency";
import { useAssets } from "@packages/components/providers/AssetsProvider";

export default function CurrencyDisplay() {
  const selectedAsset = useSelectedAsset();
  const [modal, setModal] = createSignal<boolean>(false);
  const assets = useAssets();
  const assetsList = createMemo(() => [...assets().values()]);

  return (
    <div>
      <div class="px-4 pt-4">
        <div class="flex flex-row justify-between">
          <div>
            <div class="flex flex-row items-center">
              <p class="text-bold font-sans text-3xl text-r-light-text dark:text-r-dark-text">
                {0.001}
              </p>
              <p class="pl-2 font-sans text-3xl text-r-dark-secondary-text">
                {selectedAsset.selectedAsset()?.symbol}
              </p>
              <ArrowDownButton class="m-1" onClick={() => setModal(true)} />
            </div>
            <p class="text-sans text-sm text-r-dark-secondary-text">
              {selectedAsset.selectedAsset()?.name}
            </p>
          </div>
          <Picture
            src={joinPaths(
              base,
              "/gfx/asset_icons/" +
                selectedAsset.selectedAsset()?.symbol.toLowerCase() +
                ".svg",
            )}
            alt="test"
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
