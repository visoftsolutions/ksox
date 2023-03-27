import { A } from "solid-start";
import { DefaultProps } from "../../interfaces";

export default function Logo(props: DefaultProps) {
  return (
    <div class="flex justify-between">
      <A href="/" class={`flex items-center gap-[16px] ${props.class}`}>
        <div>
          <img class="w-12" src="/gfx/logo.svg" alt="logo" />
        </div>
        <div class="font-inter text-logo text-white max-[250px]:hidden">
          KSOX
        </div>
      </A>
    </div>
  );
}
