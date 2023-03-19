import Header from "~/components/landing/Header";

export default function NotFound() {
  return (
    <div class="bg-background">
      <div class="m-auto flex min-h-screen max-w-7xl flex-col p-6">
        <Header />
        <div class="mt-32 flex justify-center text-center text-5xl text-white">
          We are working on that...
        </div>
      </div>
    </div>
  );
}
