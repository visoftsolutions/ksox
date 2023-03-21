import { A } from "solid-start";
import Spacing from "~/components/Spacing";
import { DefaultProps } from "../../interfaces";

export default function Section(
  props: DefaultProps & {
    sectionId?: string;
    sectionToId?: string;
    imagePath: string;
  }
) {
  return (
    <section id={props.sectionId}>
      <div class={`flex flex-col items-center justify-between ${props.class}`}>
        <div class="max-w-xl">
          <img src={props.imagePath} />
        </div>

        <Spacing class="h-8 md:w-96" />

        <div class="flex flex-col items-start gap-7">
          {props.children}

          <A
            href={`#${props.sectionToId}`}
            class="text-section-button font-bold text-links transition-colors hover:text-text-1"
          >
            Learn more
          </A>
        </div>
      </div>
    </section>
  );
}