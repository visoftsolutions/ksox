import { base } from "~/root";
import { joinPaths } from "solid-start/islands/server-router";

interface NavButtonProps {
  imgPath: string;
  name: string;
  highlighted?: boolean;
}

export default function NavButton(props: NavButtonProps) {
  return (
    <div
      class={`grid grid-rows-[auto_auto] items-center justify-center p-2 ${
        props.highlighted ? "bg-gray-3" : ""
      }`}
    >
      <div class="row-start-1 row-end-2">
        <img
          src={joinPaths(base, props.imgPath)}
          class="m-auto h-[20px] w-[20px]"
        />
      </div>
      <div class="row-start-2 row-end-3 font-lexend text-sm font-bold">
        {props.name}
      </div>
    </div>
  );
}
