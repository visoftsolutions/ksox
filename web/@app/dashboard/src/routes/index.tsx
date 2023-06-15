import { Outlet } from "solid-start";
import { SessionProvider } from "@web/components/providers/SessionProvider";
import { PrecisionProvider } from "@web/components/providers/PrecisionProvider";
import Header from "~/components/Header";
import { api } from "~/root";

export default function Index() {
  return (
    <SessionProvider api_url={api}>
      <main class="h-screen w-screen overflow-auto bg-gray-1 font-sanspro text-white">
        <div class="fixed left-0 right-0 top-0 z-20">
          <Header />
        </div>
        <div class="mb-14 mt-24">
          <PrecisionProvider>
            <Outlet />
          </PrecisionProvider>
        </div>
      </main>
    </SessionProvider>
  );
}
