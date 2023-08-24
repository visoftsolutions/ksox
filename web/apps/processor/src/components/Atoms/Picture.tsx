import { Match, Switch } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export interface IPicture {
  src: string;
  alt?: string;
  size?: number;
  class?: string;
  skeleton?: boolean;
}

export default function Picture(props: IPicture) {
  return (
    <div class={`rounded-full aspect-square ${props.class}`}>
      <Switch>
        <Match when={!props.skeleton}>
          <img
            src={joinPaths(base, props.src)}
            alt={props.alt || ""}
            style={{
              width: `${props.size || 42}px`,
              height: `${props.size || 42}px`,
            }}
          />
        </Match>
        <Match when={props.skeleton}>
          <div
            style={{
              width: `${props.size || 42}px`,
              height: `${props.size || 42}px`,
            }}
            class="bg-r-blue-light-backdrop dark:bg-r-blue-dark-backdrop aspect-square rounded-full"
          />
        </Match>
      </Switch>
    </div>
  );
}
