import { A } from "solid-start";
import { DefaultProps } from "../../interfaces";

export default function Logo(props: DefaultProps) {
  return (
    <A href="/" class={`flex items-center gap-[16px] ${props.class}`}>
      <div>
        <img class="w-12" src="/gfx/logo.png" />
      </div>
      <div class="font-inter text-logo text-white max-[250px]:hidden">KSOX</div>
    </A>
  );
}
