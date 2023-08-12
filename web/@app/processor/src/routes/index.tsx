import { Outlet } from "solid-start";
import { SessionProvider } from "@web/components/providers/SessionProvider";
import { PrecisionProvider } from "@web/components/providers/PrecisionProvider";
import Navigation from "~/components/Navigation";
import { api } from "~/root";

import Header from "~/components/Header/Header";
import AccountDashboard from "~/components/AccountDashboard";

export default function Index() {
  return (
    <SessionProvider api_url={api}>
      <main class="h-screen w-screen overflow-auto bg-r-dark-background font-sanspro text-white">
        <div class="fixed left-0 right-0 top-0 z-20">
          <Header />
          <AccountDashboard />
        </div>
        <div class="mb-14 mt-24">
          <PrecisionProvider>
            <Outlet />
          </PrecisionProvider>
        </div>
        <div class="fixed bottom-0 left-0 right-0 z-20">
          <Navigation />
        </div>
      </main>
    </SessionProvider>
  );
}
