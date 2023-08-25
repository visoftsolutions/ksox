import { createSignal } from "solid-js";
import Currencies from "~/components/Atoms/Currencies/Currencies";

export default function Modal() {
  const [isVisible, setIsVisible] = createSignal(false);

  return (
    <div class="fixed inset-0 z-50 flex justify-center xl:grid xl:grid-cols-3 xl:grid-rows-3">
      {/* Backdrop */}
      <div
        class={`fixed inset-0 bg-black opacity-50 ${
          isVisible() ? "transition-opacity duration-1000" : ""
        }`}
      />

      {/* Modal */}
      <div
        class={`absolute bottom-0 xl:bottom-auto w-full rounded-t-xl bg-r-light-background p-4 shadow-lg dark:bg-r-dark-foreground ${
          isVisible()
            ? "translate-y-0 transform transition-transform duration-1000"
            : "translate-y-full transform"
        } xl:col-start-2 xl:col-end-3 xl:row-start-2 xl:rounded-xl`}
      >
        {/* Modal content */}
        <h2 class="mb-4 text-lg font-semibold">Currencies</h2>
        <Currencies />
      </div>
    </div>
  );
}
