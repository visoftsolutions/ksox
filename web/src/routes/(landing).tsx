import Header from "~/components/landing/Header";
import Main from "~/components/landing/Main";
import Footer from "~/components/landing/Footer";

export default function Landing() {
  return (
    <div class="min-w-fit bg-background-landing">
      <div class="m-auto flex min-h-screen min-w-[900px] max-w-[1200px] flex-col font-lexend">
        <Header />
        <Main />
        <Footer class="mb-[170px]" />
      </div>
    </div>
  );
}
