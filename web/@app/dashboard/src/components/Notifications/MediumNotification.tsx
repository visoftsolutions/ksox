import { createEffect, createSignal } from "solid-js";
import { INotification } from "./Notification";
import { base } from "~/root";
import { joinPaths } from "solid-start/islands/server-router";

export default function MediumNotification(props: INotification) {
  const [visible, setVisible] = createSignal(true);

  createEffect(() => {
    const timerId = setTimeout(() => {
      setVisible(false);
    }, props.timer * 1000);

    return () => clearTimeout(timerId);
  });

  return (
    <div>
      {visible() && (
        <div class={`relative inset-0 m-3 mt-2 mb-2 min-h-[6rem] bg-gray-2 ${props.onAction ? "cursor-pointer" : ""}`} onClick={props.onAction}>
          <button class="absolute right-2 top-2" onClick={() => setVisible(false)}>
            <svg xmlns="http://www.w3.org/2000/svg" class="fill-current text-white" height="1.2em" viewBox="0 0 384 512">
              <path d="M342.6 150.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L192 210.7 86.6 105.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L146.7 256 41.4 361.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L192 301.3 297.4 406.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L237.3 256 342.6 150.6z" />
            </svg>
          </button>
          <div class="flex">
            <img src={joinPaths(base, props.imgPath!)} alt="" class="m-6 mr-0 h-16 w-16" />
            <p class="p-6 pr-8 text-white">{props.text}</p>
          </div>
        </div>
      )}
    </div>
  );
}
