import { AssetsProvider } from "~/components/providers/AssetsProvider";
import { MarketProvider } from "~/components/providers/MarketProvider";
import { Outlet } from "solid-start";
import MainMenu from "~/components/MainMenu";
import SideMenu from "~/components/SideMenu";
import { SessionProvider } from "@packages/components/providers/SessionProvider";
import { PrecisionProvider } from "@packages/components/providers/PrecisionProvider";
import { api } from "~/root";

export default function Index() {
  return (
    <SessionProvider api_url={api}>
      <MarketProvider>
        <main class="grid h-screen w-screen grid-cols-[72px_240px_1fr_260px_260px] grid-rows-[48px_1fr_280px] gap-[1px] overflow-auto bg-gray-1 font-sanspro text-white">
          <div class="col-start-2 col-end-6 row-start-1 row-end-2 bg-gray-2">
            <MainMenu />
          </div>
          <div class="col-start-1 col-end-2 row-start-1 row-end-4 bg-gray-1">
            <SideMenu />
          </div>
          <AssetsProvider>
            <PrecisionProvider>
              <Outlet />
            </PrecisionProvider>
          </AssetsProvider>
        </main>
      </MarketProvider>
    </SessionProvider>
  );
}
