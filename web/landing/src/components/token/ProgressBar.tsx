import { createEffect, onMount } from "solid-js";

export interface ProgressBarProps {
    fill: number;
}

export default function ProgressBar (props: ProgressBarProps) {
    let sliderDOM!: HTMLInputElement;
    let popupDOM!: HTMLInputElement;

    function popupPosition(sliderDOM: HTMLInputElement, popupDOM: HTMLElement, value: number) {
        const sliderRect = sliderDOM.getBoundingClientRect();
        const popupRect = popupDOM.getBoundingClientRect();
        return Math.max(
        Math.min(
            sliderRect.width * value - popupRect.width / 2,
            sliderRect.width - popupRect.width
        ),0
        );
    }

    createEffect(() => {
        const popupRect = popupDOM.getBoundingClientRect();
        popupDOM.style.top = - popupRect.height - 5 + "px";
        const left = popupPosition(sliderDOM, popupDOM, props.fill);
        popupDOM.style.left = left + "px";
    })

    const handler = () => {
        const popupRect = popupDOM.getBoundingClientRect();
        popupDOM.style.top = - popupRect.height - 10 + "px";
        const left = popupPosition(sliderDOM, popupDOM, props.fill);
        popupDOM.style.left = left + "px";
    }

    onMount(() => {
        window.addEventListener('resize', handler);
      });

    return (
        <div class="relative rounded-sm h-8 stripes" ref={sliderDOM}>
            <div class="absolute text-white font-bold text-xs duration-100 transition-all" ref={popupDOM}>{((Math.min(1, Math.max(0, props.fill)))*100).toFixed(2)+"%"}</div>
            <div class="absolute left-0 top-0 bottom-0 rounded-sm token-linear-wipe-button duration-100 transition-all"
                style={{right: ((1-Math.min(1, Math.max(0, props.fill)))*100).toFixed(2)+"%"}}
            />
        </div>
    );
}