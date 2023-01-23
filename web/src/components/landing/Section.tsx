import { A } from "solid-start";
import { DefaultProps } from "../interfaces";

export default function Section(
  props: DefaultProps & {
    sectionId?: string;
    sectionToId?: string;
    imagePath: string;
    imageClass?: string;
    text0: string;
    text1: string;
  }
) {
  return (
    <section id={props.sectionId}>
      <div class={`${props.class} flex justify-between gap-[32px]`}>
        <img class={props.imageClass} src={props.imagePath} />
        <div class="flex flex-col items-start gap-[27px]">
          <span class="text-section-beginning font-bold text-secondary">
            {props.text0}
          </span>

          <span class="text-section-title font-bold text-text-white">
            {props.text1}
          </span>

          <span class="text-section-main font-light text-text-light">
            {props.children}
          </span>

          <A
            href={`#${props.sectionToId}`}
            class="text-section-button font-medium text-text-faded transition-colors hover:text-text-white"
          >
            Learn more
          </A>
        </div>
      </div>
    </section>
  );
}
