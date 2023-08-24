export interface IActionButton {
  text?: string;
  onClick?: () => void
}

export default function ActionButton(props: IActionButton) {
  return (
    <div
      class={`cursor-pointer border-r-light-text dark:border-r-dark-text border rounded-xl grid items-center justify-center justify-items-center font-bold text-sm p-2`}
      onClick={props.onClick}
    >
      <div class="text-ellipsis">{props.text}</div>
    </div>
  );
}
