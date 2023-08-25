import Currency from "~/components/Atoms/Currencies/Currency";
import { useAssets } from "@packages/components/providers/AssetsProvider";
import { useSelectedAsset } from "~/components/providers/SelectedAssetProvider";
import { Index, createMemo } from "solid-js";

export default function Currencies() {
  const assets = useAssets();
  const assetsList = createMemo(() => [...assets().values()]);
  const selectedAssets = useSelectedAsset();

  return (
    <div class="relative rounded-xl bg-r-light-foreground dark:bg-r-dark-modal-foreground p-1 max-h-[40vh] overflow-scroll overflow-y-auto scrollbar-thumb-r-dark-secondary-text dark:scrollbar-thumb-r-dark-active">
      <Index each={assetsList()}>
        {(element) => (
          <Currency
            asset={element()}
            selected={selectedAssets.selectedAsset() == element()}
            onClick={() => {
              selectedAssets.setSelectedAsset(element());
            }}
          />
        )}
      </Index>
    </div>
  );
}
