import { JSXElement } from "solid-js";
import Spacing from "~/components/Spacing";

export default function ContactItem(props: {
  imageSrc: string;
  children?: JSXElement;
}) {
  return (
    <div class="flex flex-col items-center">
      <div class="flex flex-col items-center">
        <div>
          <img src={props.imageSrc} />
        </div>
        <Spacing class="h-3" />
        {props.children}
      </div>
    </div>
  );
}
