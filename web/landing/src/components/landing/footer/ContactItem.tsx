import { JSXElement } from "solid-js";
import Spacing from "~/components/Spacing";

export default function ContactItem(props: {
  imageSrc: string;
  children?: JSXElement;
}) {
  return (
    <div class="flex flex-col items-center md:flex-row md:gap-2">
      <div class="flex flex-col items-center md:flex-row md:gap-2 ">
        <div>
          <img src={props.imageSrc} alt="media" />
        </div>
        <Spacing class="h-3" />
        {props.children}
      </div>
    </div>
  );
}
