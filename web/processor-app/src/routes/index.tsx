import { AssetsProvider } from "~/utils/providers/AssetsProvider";
import { Outlet } from "solid-start";
import { SessionProvider } from "~/utils/providers/SessionProvider";
import { PrecisionProvider } from "~/utils/providers/PrecisionProvider";
import Header from "~/components/Header";
import Navigation from "~/components/Navigation";
import { NavProvider } from "~/utils/providers/NavProvider";

export default function Index() {
  return (
    <SessionProvider>
      <NavProvider>
        <main class="grid h-screen w-screen grid-rows-[auto_1fr_auto] overflow-auto bg-gray-1 font-sanspro text-white">
          <Header />
          <AssetsProvider>
            <PrecisionProvider>
              <Outlet />
            </PrecisionProvider>
          </AssetsProvider>
          <Navigation />
        </main>
      </NavProvider>
    </SessionProvider>
  );
}
