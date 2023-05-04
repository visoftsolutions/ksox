import { Outlet } from "solid-start";
import Footer from "~/components/Footer";
import Header from "~/components/Header";
import Spacing from "~/components/Spacing";
import { NavProvider } from "~/utils/providers/NavProvider";

export default function Index() {
  return (
    <div class="[background-image:linear-gradient(180deg,#000033_0%,#00001d_24%,#00001d_76%,#000033_100%)]">
      <div class="m-auto flex min-h-screen max-w-7xl flex-col p-6 ">
        <NavProvider>
          <Header />
          <Spacing class="h-12" />
          <Outlet />
          <Spacing class="h-12" />
          <Footer />
          <Spacing class="h-20" />
        </NavProvider>
      </div>
    </div>
  );
}
