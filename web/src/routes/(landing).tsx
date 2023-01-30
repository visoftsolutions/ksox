import Footer from "~/components/landing/Footer";
import Header from "~/components/landing/Header";
import Main from "~/components/landing/Main";
import Spacing from "~/components/Spacing";

export default function Landing() {
  return (
    <div class="bg-background">
      <div class="m-auto flex min-h-screen max-w-7xl flex-col p-6">
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
