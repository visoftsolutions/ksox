import { JSX } from "solid-js";

export interface SliderDisplay {
  value: number;
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  disabled?: boolean;
  disabledClass?: JSX.HTMLAttributes<HTMLElement>["class"];
  onInput?: (e: Event) => void;
}

export default function Slider(props: SliderDisplay) {
  let sliderDOM!: HTMLInputElement;
  let popupDOM!: HTMLInputElement;

  function popupPosition(sliderDOM: HTMLInputElement, popupDOM: HTMLElement) {
    const sliderRect = sliderDOM.getBoundingClientRect();
    const popupRect = popupDOM.getBoundingClientRect();
    return Math.max(
      Math.min(
        sliderRect.left + (sliderRect.width * sliderDOM.valueAsNumber) / 100 - popupRect.width / 2,
        sliderRect.left + sliderRect.width - popupRect.width
      ),
      sliderRect.left
    );
  }

  function handler(this: HTMLInputElement) {
    popupDOM.innerHTML = Math.floor(this.valueAsNumber).toString() + "%";
    const left = popupPosition(this, popupDOM);
    popupDOM.style.left = left + "px";
  }

  function mouseDown() {
    popupDOM.style.display = "block";
    popupDOM.innerHTML = Math.floor(sliderDOM.valueAsNumber).toString() + "%";
    const sliderRect = sliderDOM.getBoundingClientRect();
    const popupRect = popupDOM.getBoundingClientRect();
    popupDOM.style.top = sliderRect.top - popupRect.height - 5 + "px";
    const left = popupPosition(sliderDOM, popupDOM);
    popupDOM.style.left = left + "px";
    sliderDOM.addEventListener("input", handler);
  }
  function mouseUp() {
    popupDOM.style.display = "none";
    sliderDOM.removeEventListener("input", handler);
  }

  return (
    <div class={`relative ${props.class} ${props.disabled ? props.disabledClass : ""}`}>
      <div class="fixed hidden rounded-sm bg-slate-700 px-[8px] py-[4px] text-submit-sublabel" ref={popupDOM} />
      <div class="flex-1 text-center">
        <input
          class="slider w-full appearance-none"
          type="range"
          min="0"
          max="100"
          step={0.0001}
          value={props.value}
          ref={sliderDOM}
          disabled={props.disabled}
          onMouseDown={() => {
            mouseDown();
            console.log("down");
          }}
          onMouseUp={() => {
            mouseUp();
            console.log("onMouseUp");
          }}
          onInput={(e) => {
            if (props.onInput != undefined) {
              props.onInput(e);
            }
          }}
        />
      </div>
    </div>
  );
}
