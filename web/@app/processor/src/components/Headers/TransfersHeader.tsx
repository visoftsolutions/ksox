import { createSignal } from "solid-js";
import Button from "../Atoms/Buttons/Button";
import Picture from "../Atoms/Picture";
import SearchBar from "../Atoms/SearchBar";
import PlusIcon from "../Atoms/Icons/PlusIcon";
import SlidingModal from "../Atoms/Modals/SlidingModal";

export default function TransfersHeader() {
  return (
    <div class="flex flex-col m-6 mb-2 justify-center">
      <div class="flex justify-between">
        <Picture
          src="gfx/bitcoin_placeholder.png"
          alt="test"
          size={32}
        />
        <p class="text-r-light-text dark:text-r-dark-text font-sans font-bold">Transfer</p>
        <Button icon={PlusIcon({size: "24px"})} text="New" height="22px" textColor="white" buttonClass="m-0 p-1" color="bg-r-blue"/>
      </div>
      <SearchBar placeholder="Search" />
    </div>
  );
}
