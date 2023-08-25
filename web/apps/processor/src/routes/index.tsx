import { Outlet } from "solid-start";
import { SessionProvider } from "@packages/components/providers/SessionProvider";
import { PrecisionProvider } from "@packages/components/providers/PrecisionProvider";
import SideNav from "~/components/SideNav";
import Nav from "~/components/Nav";
import {
  AssetsProvider,
  useAssets,
} from "@packages/components/providers/AssetsProvider";
import { api } from "~/root";
import { SelectedAssetProvider, selectedAsset, setSelectedAsset } from "~/components/providers/SelectedAssetProvider";
import { createEffect, createMemo } from "solid-js";

export default function Index() {
  const assets = useAssets();
  const assetsList = createMemo(() => [...assets().values()]);

  createEffect(() => {
    if (selectedAsset() == undefined && assetsList().length > 0) {
      setSelectedAsset(assetsList()[0])
    }
  })

  return (
    <SessionProvider api_url={api}>
      <AssetsProvider api_url={api}>
        <SelectedAssetProvider>
          <main
            class="h-screen w-screen bg-r-light-background dark:bg-r-dark-background font-sanspro
          text-r-light-text dark:text-r-dark-text grid grid-rows-[1fr_auto]"
          >
            <div class="grid grid-cols-1 xl:grid-cols-[256px_1fr_256px] row-start-1 row-end-2 p-6">
              <div class="hidden xl:block">
                <SideNav />
              </div>
              <div class="xl:col-span-1">
                <div class="max-w-xl mx-auto h-full">
                  <PrecisionProvider>
                    <Outlet />
                  </PrecisionProvider>
                </div>
              </div>
            </div>
            <div class="xl:hidden block row-start-2 row-end-3">
              <Nav />
            </div>
          </main>
        </SelectedAssetProvider>
      </AssetsProvider>
    </SessionProvider>
  );
}
