import Header from "~/components/landing/Header";

export default function NotFound() {
  return (
    <div class="min-w-fit bg-background-landing">
      <div class="m-auto flex min-h-screen min-w-[900px] max-w-[1200px] flex-col">
        <Header />
        <div class="flex h-screen items-center justify-center bg-background-landing">
          <span class="text-text-white [font-family:'Lexend'] [font-style:normal] [font-weight:700] [font-size:36px] [line-height:48px]">
            We are working on that...
          </span>
        </div>
      </div>
    </div>
  );
}
