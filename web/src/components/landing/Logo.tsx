import { A } from "solid-start";

export default function Logo() {
  return (
    <A href="/" class="flex items-center gap-[16px]">
      <img width={56} src="/logo.png" />
      <span class="font-inter text-logo text-white">KSOX</span>
    </A>
  );
}
