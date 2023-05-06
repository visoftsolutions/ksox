import { createEffect, onMount } from "solid-js";

export interface ProgressBarProps {
    fill: number;
    disable: boolean
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
        popupDOM.style.bottom = "-16px";
        const left = popupPosition(sliderDOM, popupDOM, !props.disable ? props.fill : 0);
        popupDOM.style.left = left + "px";
    })

    const handler = () => {
        popupDOM.style.bottom = "-16px";
        const left = popupPosition(sliderDOM, popupDOM, !props.disable ? props.fill : 0);
        popupDOM.style.left = left + "px";
    }

    onMount(() => {
        window.addEventListener('resize', handler);
        popupDOM.style.display = "Block";
        handler();
    });

    return (
        <div class={`relative rounded-sm h-8 stripes ${!props.disable ? "animate-stripes" : ""}`} ref={sliderDOM}>
            <div class={`absolute font-bold text-xs duration-100 hidden ${!props.disable ? "text-white" : "text-gray-700"}`} ref={popupDOM}>{((Math.min(1, Math.max(0, !props.disable ? props.fill : 0)))*100).toFixed(2)+"%"}</div>
            <div class={`absolute left-0 top-0 bottom-0 rounded-sm ${!props.disable ? "token-linear-wipe-button duration-100" : "text-gray-700"}`}
                style={{right: ((1-Math.min(1, Math.max(0, !props.disable ? props.fill : 0)))*100).toFixed(2)+"%"}}
            />
        </div>
    );
}