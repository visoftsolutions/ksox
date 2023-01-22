import Header from "~/components/landing/Header";
import Main from "~/components/landing/Main";
import Footer from "~/components/landing/Footer";

export default function Landing() {
  return (
    <div class="bg-background-landing min-w-fit">
      <div class="max-w-[1200px] min-w-[900px] min-h-screen m-auto flex flex-col">
        <Header />
        <Main />
        <Footer class="mb-[170px]" />
      </div>
    </div>
  );
}
