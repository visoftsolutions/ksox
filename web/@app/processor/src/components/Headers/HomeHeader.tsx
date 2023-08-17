import { createEffect } from "solid-js";
import Button from "../Atoms/Buttons/Button";
import DepositIcon from "../Atoms/Icons/DepositIcon";
import NotificationsButton from "../Atoms/Notifications/NotificationsButton";
import Picture from "../Atoms/Picture";
import SearchBar from "../Atoms/SearchBar";
import { Palette } from "../Atoms/Palette";
import WithdrawIcon from "../Atoms/Icons/WithdrawIcon";

export default function HomeHeader() {
  return (
    <div class="flex flex-col m-6 mb-2 justify-center">
      <div class="flex justify-between">
        <Picture
          src="gfx/bitcoin_placeholder.png"
          alt="test"
          size={32}
        />
        <p class="text-r-light-text dark:text-r-dark-text font-sans font-bold">Home</p>
        <NotificationsButton status={true} />
      </div>
      <SearchBar placeholder="Search" />
    </div>
  );
}
