import { createMemo, JSX } from "solid-js";

export interface SearchInputComponent {
  value?: string;
  left?: JSX.Element;
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  disabled?: boolean;
  disabledClass?: JSX.HTMLAttributes<HTMLElement>["class"];
  onInput?: (e: Event) => void;
  onChange?: (e: Event) => void;
}

export default function SearchInput(props: SearchInputComponent) {
  let inputDOM!: HTMLInputElement;
  const valueDOM = createMemo(() => {
    return props.value == null || props.value == undefined ? "" : props.value;
  });

  return (
    <div
      class={`grid cursor-text grid-cols-[auto_1fr] items-center rounded-md bg-gray-1 p-1 ${props.class} ${props.disabled ? props.disabledClass : ""}`}
      onClick={() => {
        inputDOM.focus();
      }}
    >
      <div class="col-start-1 col-end-2 px-[8px] text-gray-4">{props.left}</div>
      <div class="col-start-2 col-end-3 text-right">
        <input
          class={"w-full bg-transparent p-1 text-left placeholder-gray-4 outline-none"}
          type="text"
          spellcheck={true}
          ref={inputDOM}
          value={valueDOM()}
          placeholder="Search"
          disabled={props.disabled}
          onInput={(ev) => (props.onInput ? props.onInput(ev) : {})}
          onChange={(ev) => (props.onChange ? props.onChange(ev) : {})}
          onFocus={(e) => {
            (e.target as HTMLInputElement).select();
          }}
        />
      </div>
    </div>
  );
}
