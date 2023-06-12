import { For } from "solid-js";
import Badge from "./Badge"

interface BadgeProperties {
  name: string;
  description: string;
  imageURL: string;
  progress: number
}

export default function BadgePanel(props: {badgeData: BadgeProperties[][]}) {
  return <div class="flex flex-wrap w-3/4 justify-center">
    <For each={props.badgeData}>{(category, _) =>
      <div 
        class="flex gap-y-4 flex-col px-4 border-r-4 border-l-4 border-dashed border-gray-4"
        style={{ "margin-left": "-4px" }}
      >
        <For each={category}>{(badge, _) =>
          <Badge 
            name={badge.name} 
            description={badge.description}
            imageURL={badge.imageURL}
            progress={badge.progress}
          ></Badge>
        }</For>
      </div>
    }</For>
  </div>;
}