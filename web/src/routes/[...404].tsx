import Header from "~/components/landing/Header";

export default function NotFound() {
  return (
    <div class="min-w-fit bg-background-landing">
      <div class="m-auto flex min-h-screen min-w-[900px] max-w-[1200px] flex-col">
        <Header />
        <div class="mt-32 flex h-screen justify-center bg-background-landing">
          <span class="text-5xl text-text-white">
            We are working on that...
          </span>
        </div>
      </div>
    </div>
  );
}
