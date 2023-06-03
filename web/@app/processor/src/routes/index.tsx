import { AssetsProvider } from "~/components/providers/AssetsProvider";
import { Outlet } from "solid-start";
import { SessionProvider } from "@web/components/providers/SessionProvider";
import { PrecisionProvider } from "@web/components/providers/PrecisionProvider";
import Header from "~/components/Header";
import Navigation from "~/components/Navigation";
import { NavProvider } from "~/components/providers/NavProvider";
import { AssetProvider } from "~/components/providers/AssetProvider";

export default function Index() {
  return (
    <SessionProvider>
      <NavProvider>
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
      </NavProvider>
    </SessionProvider>
  );
}
