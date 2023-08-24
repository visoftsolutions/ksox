import { createUniqueId, JSX } from "solid-js"

export interface TextInputComponent {
  value?: string
  left?: JSX.Element
  right?: JSX.Element
  class?: JSX.HTMLAttributes<HTMLElement>["class"]
  disabled?: boolean
  disabledClass?: JSX.HTMLAttributes<HTMLElement>["class"]
  precision?: number
  onInput?: (f: string) => void
  onChange?: (f: string) => void
}

export function DepositTextInput(props: TextInputComponent) {
  let inputDOM!: HTMLInputElement

  const id = createUniqueId()

  return (
    <div
      class={`cursor-text py-2 items-center rounded-md bg-gray-2 flex w-full pr-2 ${
        props.class
      } ${props.disabled ? props.disabledClass : ""}`}
      onClick={() => {
        inputDOM.focus()
      }}
    >
      <label
        for={id}
        class="min-w-[50px] px-[8px] text-left text-submit-sublabel text-gray-4"
      >
        To
      </label>
      <div class="w-full">
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
              props.onInput(e.target.value)
            }
          }}
          onChange={(e) => {
            if (props.onChange != undefined) {
              props.onChange(e.target.value)
            }
          }}
          onFocus={(e) => {
            ;(e.target as HTMLInputElement).select()
          }}
        />
      </div>
      {/*   <div class=" min-w-[50px] px-[8px] text-right text-submit-sublabel text-gray-4">
        {props.right}
      </div> */}
    </div>
  )
}
