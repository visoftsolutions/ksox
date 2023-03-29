import { createMemo, JSX } from "solid-js";

export interface NumberInputComponent {
  value?: string;
  left?: JSX.Element;
  right?: JSX.Element;
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  disabled?: boolean;
  disabledClass?: JSX.HTMLAttributes<HTMLElement>["class"];
  precision?: number;
  onInput?: (e: Event) => void;
  onChange?: (e: Event) => void;
}

function formatInput(input: string, precision: number) {
  // Remove all non-digit and non-dot characters
  let formatted = precision > 0 ? input.replace(/[^0-9.]/g, "") : input.replace(/[^0-9]/g, "");
  formatted = formatted.replace(/(\..*?)\..*/g, "$1").replace(/^0[^.]/, "0");

  // Truncate decimal part to maxDigits digits
  formatted = formatted.replace(new RegExp("(\\.\\d{0," + precision + "})\\d*"), "$1");

  // Add commas every third digit before the dot
  const dotIndex = formatted.indexOf(".");
  if (dotIndex !== -1) {
    const integerPart = formatted.substring(0, dotIndex).replace(/\B(?=(\d{3})+(?!\d))/g, ",");
    const decimalPart = formatted.substring(dotIndex);
    formatted = integerPart + decimalPart.replace(/(?<=\.)[^.]+/g, (match) => match.replace(/\./g, ""));
  } else {
    formatted = formatted.replace(/\B(?=(\d{3})+(?!\d))/g, ",");
  }

  // Ensure there are no two commas next to each other
  formatted = formatted.replace(/(,)\1+/g, "$1");

  return formatted;
}

export default function NumberInput(props: NumberInputComponent) {
  let inputDOM!: HTMLInputElement;
  const valueDOM = createMemo(() => {
    return props.value == undefined ? "" : props.value;
  });

  return (
    <div
      class={`grid cursor-text grid-cols-[auto_1fr_auto] items-center rounded-md ${props.class} ${props.disabled ? props.disabledClass : ""}`}
      onClick={() => {
        inputDOM.focus();
      }}
    >
      <div class="col-start-1 col-end-2 min-w-[50px] px-[8px] text-left text-submit-sublabel text-gray-4">{props.left}</div>
      <div class="col-start-2 col-end-3 text-right">
        <input
          class={"number_input w-full max-w-[150px] bg-transparent p-1 text-right outline-none"}
          type="text"
          spellcheck={true}
          ref={inputDOM}
          value={valueDOM()}
          disabled={props.disabled}
          onInput={(e) => {
            (e.target as HTMLInputElement).value = formatInput((e.target as HTMLInputElement).value, props.precision != undefined ? props.precision : 0);
            if (props.onInput != undefined) {
              props.onInput(e);
            }
          }}
          onChange={(e) => {
            if (props.onChange != undefined) {
              props.onChange(e);
            }
          }}
          onFocus={(e) => {
            (e.target as HTMLInputElement).select();
          }}
        />
      </div>
      <div class="col-start-3 col-end-4 min-w-[50px] px-[8px] text-right text-submit-sublabel text-gray-4">{props.right}</div>
    </div>
  );
}
