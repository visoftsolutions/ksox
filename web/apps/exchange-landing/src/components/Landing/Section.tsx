import { JSX } from "solid-js";
import { A } from "solid-start";
import Spacing from "~/components/Spacing";

interface DefaultProps {
  id?: string;
  style?: JSX.CSSProperties;
  class?: string;
  children?: JSX.Element;
}

export default function Section(
  props: DefaultProps & {
    sectionId?: string;
    sectionToId?: string;
    imagePath: string;
  },
) {
  return (
    <section id={props.sectionId} class="scroll-mt-36">
      <div class={`flex flex-col items-center justify-between ${props.class}`}>
        <div class="flex-1">
          <img
            src={props.imagePath}
            alt="about"
            elementtiming={""}
            fetchpriority={"high"}
          />
        </div>

        <Spacing class="h-8 md:w-5" />

        <div class="flex flex-1 flex-col items-start gap-7">
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
