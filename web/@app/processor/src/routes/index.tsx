import { Outlet } from "solid-start";
import { SessionProvider } from "@web/components/providers/SessionProvider";
import { PrecisionProvider } from "@web/components/providers/PrecisionProvider";
import Navigation from "~/components/Navigations/Navigation";
import { api } from "~/root";

import Header from "~/components/Headers/HomeHeader";
import AccountDashboard from "~/components/AccountDashboard";
import SideNavigation from "~/components/Navigations/SideNavigation";
import Profile from "~/components/Atoms/Profile";

export default function Index() {
  return (
    <SessionProvider api_url={api}>
      <main class="h-screen w-screen overflow-auto bg-r-light-background dark:bg-r-dark-background font-sanspro text-r-light-text dark:text-r-dark-text">
        <div class="grid grid-cols-1 xl:grid-cols-3">
          {/* First Column: Navigation and Header */}
          {/* <div class="xl:col-span-1 fixed left-0 right-0 top-0 z-20">
          </div> */}
          <div class="hidden xl:block m-6">
            <div class="mb-6 ml-2">
            <Profile className="" name="Filip Dziurdzia" img="gfx/bitcoin_placeholder.png" />
            </div>
            <SideNavigation />
          </div>
          
          {/* Second Column: Outlet */}
          <div class="xl:col-span-1">
            <PrecisionProvider>
              <Outlet />
            </PrecisionProvider>
          </div>
          
          {/* Third Column: Empty Placeholder */}
          <div class="xl:col-span-1"></div>
        </div>
        
        {/* Fixed Navigation at the Bottom */}
        <div class="fixed bottom-0 left-0 right-0 z-20 xl:hidden">
          <Navigation />
        </div>
      </main>
    </SessionProvider>
  );
}
