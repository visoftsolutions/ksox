import { createEffect } from "solid-js";
import Button from "../Atoms/Buttons/Button";
import DepositIcon from "../Atoms/Icons/DepositIcon";
import NotificationsButton from "../Atoms/Buttons/NotificationsButton";
import Picture from "../Atoms/Picture";
import SearchBar from "../Atoms/SearchBar";
import { Palette } from "../Atoms/Palette";
import WithdrawIcon from "../Atoms/Icons/WithdrawIcon";
import { useDarkModeContext } from "../providers/DarkModeProvider";

export default function HomeHeader() {

  const darkMode = useDarkModeContext();
  return (
    <div class="grid grid-cols-3 gap-4 m-6 mb-2 justify-center">
      <div class="flex items-center" onClick={() => darkMode.setDarkMode(!darkMode.darkMode())}>
        <Picture src="gfx/bitcoin_placeholder.png" alt="test" size={32} />
      </div>
      <div class="flex items-center justify-center">
        <p class="text-r-light-text dark:text-r-dark-text font-sans font-bold">Home</p>
      </div>
      <div class="flex items-center justify-end">
        <NotificationsButton status={true} />
      </div>
      <div class="col-span-3">
        <SearchBar placeholder="Search" />
      </div>
    </div>
  );
}
