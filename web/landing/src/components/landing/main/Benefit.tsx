import { DefaultProps } from "../../interfaces";

export default function Benefit(props: DefaultProps) {
  return (
    <div class="lg flex flex-col items-center gap-2 md:flex-row md:gap-2 lg:flex-row">
      <div>
        <img class="w-6" src="/gfx/sign.svg" alt="tick sign" />
      </div>

      <p class="md:text-hero-benefit-new text-center font-lexend text-hero-benefit font-normal text-text-1 md:font-light">
        {props.children}
      </p>
    </div>
  );
}
