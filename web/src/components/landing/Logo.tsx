import { A } from "solid-start";

export default function Logo() {
  return (
    <A href="/" class="flex items-center gap-[16px]">
      <img width={36} src="/logo.png" />
      <span class="text-white [font-style:normal] [font-weight:400] [font-family:'Inter'] [font-size:32px] [line-height:39px]">
        KSOX
      </span>
    </A>
  );
}
