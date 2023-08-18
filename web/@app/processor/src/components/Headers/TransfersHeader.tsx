import { createSignal } from "solid-js";
import Button from "../Atoms/Buttons/Button";
import Picture from "../Atoms/Picture";
import SearchBar from "../Atoms/SearchBar";
import PlusIcon from "../Atoms/Icons/PlusIcon";
import SlidingModal from "../Atoms/Modals/Modal";
import NewButton from "../Atoms/Buttons/NewButton";
import { useDarkModeContext } from "../providers/DarkModeProvider";

export default function TransfersHeader() {
  const darkMode = useDarkModeContext();
  return (
    <div class="m-6 mb-2 grid grid-cols-3 justify-center gap-4">
      <div
        class="flex items-center"
        onClick={() => darkMode.setDarkMode(!darkMode.darkMode())}
      >
        <Picture src="gfx/bitcoin_placeholder.png" alt="test" size={32} />
      </div>
      <div class="flex items-center justify-center">
        <p class="font-sans font-bold text-r-light-text dark:text-r-dark-text">
          Transfer
        </p>
      </div>
      <div class="flex items-center justify-end">
        <NewButton onClick={() => {}} />
      </div>
      <div class="col-span-3">
        <SearchBar placeholder="Search" />
      </div>
    </div>
  );
}
