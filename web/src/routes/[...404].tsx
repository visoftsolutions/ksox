import Header from "~/components/landing/Header";

export default function NotFound() {
  return (
    <div class="bg-background-landing min-w-fit">
      <div class="max-w-[1200px] min-w-[900px] min-h-screen m-auto flex flex-col">
        <Header />
        <div class="bg-background-landing h-screen flex items-center justify-center">
          <span class="text-text-white [font-family:'Lexend'] [font-style:normal] [font-weight:700] [font-size:36px] [line-height:48px]">
            We are working on that...
          </span>
        </div>
      </div>
    </div>
  );
}
