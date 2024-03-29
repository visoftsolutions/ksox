import { Outlet } from "solid-start";
import Footer from "~/components/Footer";
import Header from "~/components/Header";
import Spacing from "~/components/Spacing";
import { CrowdsaleProvider } from "~/components/providers/CrowdsaleProvider";

export default function Index() {
  return (
    <div class="[background-image:linear-gradient(180deg,#000033_0%,#00001d_24%,#00001d_76%,#000033_100%)]">
      <CrowdsaleProvider>
        <Header />
        <div class="m-auto flex min-h-screen max-w-7xl flex-col p-6">
          <Spacing class="h-24" />
          <Outlet />
          <Spacing class="h-12" />
          <Footer />
          <Spacing class="h-20" />
        </div>
      </CrowdsaleProvider>
    </div>
  );
}
