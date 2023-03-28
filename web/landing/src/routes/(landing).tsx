import Footer from "~/components/Footer";
import Header from "~/components/Header";
import Main from "~/components/Main";
import Spacing from "~/components/Spacing";

export default function Landing() {
  return (
    <div class="[background-image:linear-gradient(180deg,#000033_0%,#00001d_24%,#00001d_76%,#000033_100%)]">
      <div class="m-auto flex min-h-screen max-w-7xl flex-col p-6 ">
        <Header />
        <Spacing class="h-12" />
        <Main />
        <Spacing class="h-12" />
        <Footer />
        <Spacing class="h-20" />
      </div>
    </div>
  );
}
