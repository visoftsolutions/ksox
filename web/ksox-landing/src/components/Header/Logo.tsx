import { A } from "solid-start";
import { DefaultProps } from "~/utils/interfaces";

export default function Logo(props: DefaultProps) {
  return (
    <div class={`flex justify-between ${props.class}`}>
      <A href="/" class="flex items-center gap-[16px]">
        <div>
          <img class="w-12" src="/gfx/logo.svg" alt="logo" />
        </div>
        <div class="font-inter text-logo text-white max-sm:hidden">KSOX</div>
      </A>
    </div>
  );
}
