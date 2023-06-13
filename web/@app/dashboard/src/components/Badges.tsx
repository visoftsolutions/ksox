import { Index } from "solid-js";
import Badge from "./Badges/Badge";

export interface BadgesElements {
  name: string;
  description: string;
  progress: number;
}

export interface BadgesProps {
  badges?: BadgesElements[];
}

export default function Badges(props: BadgesProps) {
  return (
    <div class="grid grid-cols-[repeat(auto-fit,minmax(300px,1fr))] gap-8 md:grid-cols-[repeat(auto-fit,minmax(500px,1fr))]">
      <Index each={props.badges}>
        {(element) => (
          <Badge
            name={element().name}
            description={element().description}
            // image={`${element().name.replaceAll(" ", "").toLowerCase()}.svg`}
            progress={element().progress}
          />
        )}
      </Index>
    </div>
  );
}
