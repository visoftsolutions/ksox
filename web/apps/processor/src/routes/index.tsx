import { AssetsProvider } from "~/components/providers/AssetsProvider";
import { Outlet } from "solid-start";
import { SessionProvider } from "@packages/components/providers/SessionProvider";
import { PrecisionProvider } from "@packages/components/providers/PrecisionProvider";
import Header from "~/components/Header";
import Navigation from "~/components/Navigation";
import { AssetProvider } from "~/components/providers/AssetProvider";
import { api } from "~/root";

export default function Index() {
  return (
    <SessionProvider api_url={api}>
      <main class="h-screen w-screen overflow-auto bg-gray-1 font-sanspro text-white">
        <div class="fixed left-0 right-0 top-0 z-20">
          <Header />
        </div>
        <div class="mb-14 mt-24">
          <AssetsProvider>
            <AssetProvider>
              <PrecisionProvider>
                <Outlet />
              </PrecisionProvider>
            </AssetProvider>
          </AssetsProvider>
        </div>
        <div class="fixed bottom-0 left-0 right-0 z-20">
          <Navigation />
        </div>
      </main>
    </SessionProvider>
  );
}
