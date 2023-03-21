import { createMemo, JSX } from "solid-js";

export interface NumberInputDisplay {
  value?: number;
  left?: JSX.Element;
  right?: JSX.Element;
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  disabled?: boolean;
  disabledClass?: JSX.HTMLAttributes<HTMLElement>["class"];
  parsingFunction?: (value: number) => number;
  onInput?: (e: Event) => void;
  onChange?: (e: Event) => void;
}

export default function NumberInput(props: NumberInputDisplay) {
  let inputDOM!: HTMLInputElement;
  const valueDOM = createMemo(() => {
    return props.value == null || props.value == undefined || props.value == 0 ? "" : props.value;
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
          type="number"
          spellcheck={true}
          ref={inputDOM}
          value={valueDOM()}
          disabled={props.disabled}
          onInput={(e) => {
            if (props.parsingFunction != undefined && !isNaN((e.target as HTMLInputElement).valueAsNumber)) {
              const parsed = props.parsingFunction((e.target as HTMLInputElement).valueAsNumber);
              if ((e.target as HTMLInputElement).valueAsNumber != parsed) {
                (e.target as HTMLInputElement).valueAsNumber = parsed;
              }
            }
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
