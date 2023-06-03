import { Show, createEffect, onMount } from "solid-js";

export interface ProgressBarProps {
  fill: number | undefined;
  disable: boolean;
}

export default function ProgressBar(props: ProgressBarProps) {
  let sliderDOM!: HTMLInputElement;
  let popupDOM!: HTMLInputElement;

  function popupPosition(sliderDOM: HTMLInputElement, popupDOM: HTMLElement, value: number) {
    const sliderRect = sliderDOM.getBoundingClientRect();
    const popupRect = popupDOM.getBoundingClientRect();
    return Math.max(Math.min(sliderRect.width * value - popupRect.width / 2, sliderRect.width - popupRect.width), 0);
  }

  createEffect(() => {
    if (props.fill != undefined) {
      popupDOM.style.bottom = "-16px";
      const left = popupPosition(sliderDOM, popupDOM, !props.disable ? props.fill ?? 0 : 0);
      popupDOM.style.left = left + "px";
    }
  });

  const handler = () => {
    if (props.fill != undefined) {
      popupDOM.style.display = "Block";
      popupDOM.style.bottom = "-16px";
      const left = popupPosition(sliderDOM, popupDOM, !props.disable ? props.fill ?? 0 : 0);
      popupDOM.style.left = left + "px";
    }
  };

  onMount(() => {
    window.addEventListener("resize", handler);
    handler();
  });

  return (
    <div class={`stripes relative h-8 rounded-sm ${!props.disable ? "animate-stripes" : ""}`} ref={sliderDOM}>
      <Show when={props.fill != undefined}>
        <div class={`absolute hidden text-xs font-bold ${!props.disable ? "text-white" : "text-gray-700"}`} ref={popupDOM}>
          {(Math.min(1, Math.max(0, !props.disable ? props.fill ?? 0 : 0)) * 100).toFixed(2) + "%"}
        </div>
      </Show>
      <div
        class={`absolute bottom-0 left-0 top-0 rounded-sm ${!props.disable ? "token-linear-wipe-button " : "text-gray-700"}`}
        style={{
          right: ((1 - Math.min(1, Math.max(0, !props.disable ? props.fill ?? 0 : 0))) * 100).toFixed(2) + "%",
        }}
      />
    </div>
  );
}
