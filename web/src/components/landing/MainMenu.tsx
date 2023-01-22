import { A } from "solid-start";

export default function MainMenu() {
  return (
    <div class="flex items-center justify-between gap-[42px]">
      <A href="/about">
        <span class="text-text-faded [font-family:'Lexend'] [font-style:normal] [font-weight:500] [font-size:16px] [line-height:20px] transition-colors hover:text-text-white">
          About KSOX
        </span>
      </A>
      <A href="/contact">
        <span class="text-text-faded [font-family:'Lexend'] [font-style:normal] [font-weight:500] [font-size:16px] [line-height:20px] transition-colors hover:text-text-white">
          Contact
        </span>
      </A>
      <A
        href="/app"
        class="text-text-white bg-primary transition-colors hover:bg-text-white hover:text-primary [border-radius:100px] p-[8px_20px]"
      >
        <span class=" [font-family:'Lexend'] [font-style:normal] [font-weight:500] [font-size:16px] [line-height:20px]">
          Get Started
        </span>
      </A>
    </div>
  );
}
