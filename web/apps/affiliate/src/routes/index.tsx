import { useParams } from "solid-start";
import Footer from "~/components/Footer";
import Header from "~/components/Header";

export default function Index() {
  const params = useParams<{ code: string }>();

  return (
    <div class="[background-image:linear-gradient(180deg,#000033_0%,#00001d_24%,#00001d_76%,#000033_100%)]">
      <div class="m-auto flex min-h-screen max-w-7xl flex-col p-2 md:p-6">
        <Header />
        <div class="h-12" />
        <div class="mt-16 md:mt-32 flex justify-center text-center text-3xl md:text-5xl text-white">
          <span>Affiliate codes will be active in</span>
          <span class="text-pink-600 ml-2">2 months time</span>
        </div>
        <div class="mt-16 md:mt-32 flex justify-center text-center text-3xl md:text-5xl text-white">
          <span>Your code is</span>
          <span class="text-pink-600 ml-2">{params.code}</span>
        </div>
        {/* <Form class="mt-16 md:mt-32 flex flex-col justify-center text-center text-3xl md:text-5xl text-white">
          <TextInput
            value="type your email here"
            left="email: "
            class="m-auto"
            onInput={(e) => console.log(e)}
          />
          <div class="h-8 md:h-16" />
          <div class="m-auto main-menu-button rounded-full bg-buttonbg_new px-3 md:px-5 py-1 md:py-2 font-medium text-text-1 hover:bg-text-1 hover:text-buttonbg_new">
            Sign up
          </div>
        </Form> */}
        <div class="h-32 md:h-64" />
        <Footer />
      </div>
    </div>
  );
}
