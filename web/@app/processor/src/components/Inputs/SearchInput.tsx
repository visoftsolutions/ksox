import { createUniqueId, JSX } from "solid-js";

export interface SearchInputComponent {
  value?: string;
  left?: JSX.Element;
  class?: JSX.HTMLAttributes<HTMLElement>["class"];
  disabled?: boolean;
  disabledClass?: JSX.HTMLAttributes<HTMLElement>["class"];
  onInput?: (
    e: Event & {
      currentTarget: HTMLInputElement;
      target: HTMLInputElement;
    },
  ) => void;
  onChange?: (
    e: Event & {
      currentTarget: HTMLInputElement;
      target: HTMLInputElement;
    },
  ) => void;
}

export default function SearchInput(props: SearchInputComponent) {
  let inputDOM!: HTMLInputElement;

  const id = createUniqueId();

  return (
    <div
      class={`grid cursor-text grid-cols-[auto_1fr] items-center rounded-md bg-gray-1 p-[3px] font-sanspro text-searchInput font-normal
        ${props.class}
        ${props.disabled && props.disabledClass}
      `}
      onClick={() => {
        inputDOM.focus();
      }}
    >
      <label for={id} class="col-start-1 col-end-2 px-[6px]">
        {props.left}
      </label>
      <div class="col-start-2 col-end-3 text-right">
        <input
          id={id}
          class={
            "w-full bg-transparent p-1 text-left placeholder-gray-4 outline-none"
          }
          type="text"
          spellcheck={true}
          ref={inputDOM}
          value={props.value ?? ""}
          placeholder="Find user"
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
