import { createUniqueId, JSX } from "solid-js";

export interface AmountInputProps {
  disabled: boolean;
  onInput(e: number): void;
}

export default function AmountInput(props: AmountInputProps) {
  let inputDOM!: HTMLInputElement;

  const id = createUniqueId();

  return (
    <div class="bg-gray-1 grid grid-cols-[auto_1fr] items-center rounded-lg border-[1px] border-slate-600">
      <label
        for={id}
        class="text-submit-sublabel text-gray-4 col-start-1 col-end-2 min-w-[50px] px-[8px] text-left"
      >
        Amount:
      </label>
      <div class="col-start-2 col-end-3 text-right">
        <input
          id={id}
          class={
            "number_input w-full appearance-none bg-transparent p-1 px-3 text-right outline-none"
          }
          type="number"
          spellcheck={true}
          ref={inputDOM}
          disabled={props.disabled}
          onInput={(e) => {
            props.onInput(e.target.valueAsNumber);
          }}
        />
      </div>
    </div>
  );
}
