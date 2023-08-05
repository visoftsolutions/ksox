import { format } from "numerable";
import { createMemo, createUniqueId, JSX } from "solid-js";
import { formatTemplate } from "@web/utils/precision";

export interface TextInputComponent {
  value?: string;
  left?: JSX.Element;
  right?: JSX.Element;
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  disabled?: boolean;
  disabledClass?: JSX.HTMLAttributes<HTMLElement>["class"];
  precision?: number;
  onInput?: (f: string) => void;
  onChange?: (f: string) => void;
}

export default function TextInput(props: TextInputComponent) {
  let inputDOM!: HTMLInputElement;

  const id = createUniqueId();

  return (
    <div
      class={`grid cursor-text grid-cols-[auto_1fr_auto] items-center rounded-md bg-gray-1 ${props.class} ${props.disabled ? props.disabledClass : ""}`}
      onClick={() => {
        inputDOM.focus();
      }}
    >
      <label for={id} class="col-start-1 col-end-2 min-w-[50px] px-[8px] text-left text-submit-sublabel text-gray-4">
        {props.left}
      </label>
      <div class="col-start-2 col-end-3 text-right">
        <input
          id={id}
          class={"w-full bg-transparent p-1 text-right outline-none"}
          type="text"
          spellcheck={true}
          ref={inputDOM}
          value={props.value}
          disabled={props.disabled}
          onInput={(e) => {
            if (props.onInput != undefined) {
              props.onInput(e.target.value);
            }
          }}
          onChange={(e) => {
            if (props.onChange != undefined) {
              props.onChange(e.target.value);
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