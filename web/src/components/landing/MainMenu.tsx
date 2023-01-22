import { A } from "solid-start";

export default function MainMenu() {
  return (
    <div class="flex items-center justify-between gap-[42px]">
      <A href="/about">
        <span class="[color:#A697C6] [font-family:'Lexend'] [font-style:normal] [font-weight:500] [font-size:16px] [line-height:20px]">
          About KSOX
        </span>
      </A>
      <A href="/contact">
        <span class="[color:#A697C6] [font-family:'Lexend'] [font-style:normal] [font-weight:500] [font-size:16px] [line-height:20px]">
          Contact
        </span>
      </A>
      <A href="/app" class="bg-[#5532B9] [border-radius:100px] p-[8px_20px]">
          <span class="[color:#EBEBEB] [font-family:'Lexend'] [font-style:normal] [font-weight:500] [font-size:16px] [line-height:20px]">
            Get Started
          </span>
      </A>
    </div>
  );
}
