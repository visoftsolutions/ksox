import { JSX } from "solid-js";

export interface IHalfScreenModal {
  children: JSX.Element;
  close: () => void;
}

export default function HalfScreenModal(props: IHalfScreenModal) {
  return (
    <div
      class="fixed inset-0 z-10 grid items-end justify-items-center xl:items-center before:bg-black before:opacity-70 before:fixed before:inset-0 before:z-10"
      onClick={(event) => {
        if (event.target !== event.currentTarget) return; // Ensure the click was directly on the parent, not a child
        props.close();
      }}
    >
      <div
        class={
          "z-20 max-h-[50%] max-w-[500px] w-full rounded-t-xl xl:rounded-xl shadow-lg bg-r-light-background p-4 dark:bg-r-dark-foreground"
        }
      >
        {props.children}
      </div>
    </div>
  );
}
