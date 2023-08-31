import { useParams } from "solid-start";
import { Form } from "solid-start/data/Form";
import SubmitButton from "~/components/Buttons/Submit";
import Footer from "~/components/Footer";
import Header from "~/components/Header";
import TextInput from "~/components/Inputs/TextInput";

export default function Index() {
  const params = useParams<{ code: string }>();

  return (
    <div class="[background-image:linear-gradient(180deg,#000033_0%,#00001d_24%,#00001d_76%,#000033_100%)]">
      <div class="m-auto flex min-h-screen max-w-7xl flex-col p-6">
        <Header />
        <div class="mt-32 flex justify-center text-center text-5xl text-white">
          Affiliate codes will be active in &nbsp<span class="text-pink-600">2 months time</span>
        </div>
        <div class="mt-32 flex justify-center text-center text-5xl text-white">
          Your code is: &nbsp<span class="text-pink-600">{params.code}</span>
        </div>
        {/* <Form class="mt-32 flex flex-col justify-center text-center text-5xl text-white">
          <TextInput
            value="type your email here"
            left="email: "
            class="m-auto"
            onInput={(e) => console.log(e)}
          />
          <div class="h-16" />
          <div class="m-auto main-menu-button rounded-full bg-buttonbg_new px-5 py-2 font-medium text-text-1 hover:bg-text-1 hover:text-buttonbg_new">
            Sign up
          </div>
        </Form> */}
        <div class="h-64" />
        <Footer />
      </div>
    </div>
  );
}
