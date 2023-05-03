import { JSXElement } from "solid-js";
import Spacing from "~/components/Spacing";

export default function ContactItem(props: {
  imageSrc: string;
  children?: JSXElement;
}) {
  return (
    <div class="flex flex-col items-center md:flex-row md:gap-2">
      <div>
        <img
          src={props.imageSrc}
          alt="media"
          fetchpriority="auto"
          elementtiming="svg-image"
          width={48}
          height={48}
        />
      </div>
      <Spacing class="h-3" />
      {props.children}
    </div>
  );
}
