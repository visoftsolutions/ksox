import { DefaultProps } from "../../interfaces";

export default function Benefit(props: DefaultProps) {
  return (
    <div class={`flex items-center ${props.class}`}>
      <div>
        <img class="w-6" src="/gfx/sign.svg" />
      </div>

      <p class="text-center font-lexend text-hero-benefit font-normal text-text-1">
        {props.children}
      </p>
    </div>
  );
}