import { createEffect, createSignal } from "solid-js";
import { INotification } from "./Notification";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export default function BigNotification(props: INotification) {
  const close = () => {
    const updatedNotifications = props.notifications!().filter((n) => n.id !== props.id);
    props.setNotifications!(updatedNotifications);
  };

  const checkIfSignalPassed = () => {
    return props.notifications && props.setNotifications;
  };

  return (
    <div>
      <div class={`relative inset-0 m-3 mb-2 mt-2 min-h-[6rem] bg-gray-3 shadow ${props.onAction ? "cursor-pointer" : ""}`} onClick={props.onAction}>
      <button class="absolute right-2 top-2" onClick={() => checkIfSignalPassed() ? close() : console.log("Signal accessor and/or setter not given")}>
          <svg xmlns="http://www.w3.org/2000/svg" class="fill-current text-white" height="1.2em" viewBox="0 0 384 512">
            <path d="M342.6 150.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L192 210.7 86.6 105.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L146.7 256 41.4 361.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L192 301.3 297.4 406.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L237.3 256 342.6 150.6z" />
          </svg>
        </button>
        <div>
          <p class="p-6 text-white">{props.text}</p>
          <div class="flex flex-col items-center justify-center">
            <img src={joinPaths(base, props.imgPath!)} alt="" class="h-64 w-64" />
            <div class="h-6 w-full bg-transparent"></div>
          </div>
        </div>
      </div>
    </div>
  );
}