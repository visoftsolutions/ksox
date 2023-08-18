import { createSignal } from "solid-js";
import Button from "./Atoms/Buttons/Button";
import ArrowDownIcon from "./Atoms/Icons/ArrowDownIcon";
import { Palette } from "./Atoms/Palette";
import Picture from "./Atoms/Picture";
import SlidingModal from "./Atoms/Modals/Modal";
import { useCurrencyContext } from "./providers/CurrencyProvider";


export default function CurrencyDisplay() {
  const [isModalOpen, setIsModalOpen] = createSignal(false);

  const openModal = () => {
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
  };

  const currentCurrency = useCurrencyContext();

  return (
    <div>
      <div class="px-4 pt-4">
        <div class="flex flex-row justify-between">
          <div>
            <div class="flex flex-row items-center">
              <p class="text-bold font-sans text-3xl text-r-light-text dark:text-r-dark-text">
                {currentCurrency.currency().amount}
              </p>
              <p class="pl-2 font-sans text-3xl text-r-dark-secondary-text">
                {currentCurrency.currency().symbol}
              </p>
              <Button
                icon={ArrowDownIcon({
                  stroke: Palette["r-blue"],
                  size: "20px",
                })}
                color="bg-r-blue-light-backdrop"
                darkColor="dark:bg-r-blue-dark-backdrop"
                width="w-6"
                height="h-6"
                onClick={openModal}
                buttonClass="ml-2"
              />
              {isModalOpen() && (
                <SlidingModal
                  isOpen={isModalOpen()}
                  onClose={closeModal}
                  heightCells={8}
                />
              )}
            </div>
            <p class="text-sans text-sm text-r-dark-secondary-text">
              {currentCurrency.currency().name}
            </p>
          </div>
          <Picture src={currentCurrency.currency().img} alt="test" />
        </div>
      </div>
    </div>
  );
}
