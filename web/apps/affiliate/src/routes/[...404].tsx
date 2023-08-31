import Header from "~/components/Header";

export default function NotFound() {
  return (
    <div class="[background-image:linear-gradient(180deg,#000033_0%,#00001d_24%,#00001d_76%,#000033_100%)]">
      <div class="m-auto flex min-h-screen max-w-7xl flex-col p-6">
        <Header />
        <div class="mt-32 flex justify-center text-center text-5xl text-white">
          go back
        </div>
      </div>
    </div>
  );
}
