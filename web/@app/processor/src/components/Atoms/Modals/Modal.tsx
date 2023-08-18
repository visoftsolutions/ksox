import { createSignal } from "solid-js";
import { createEffect } from "solid-js";
import Currencies from "../Currencies/Currencies";

export interface IModal {
  isOpen: boolean;
  onClose: () => void;
  heightCells?: number;
}

export default function Modal(props: IModal) {
  const [isAnimating, setIsAnimating] = createSignal(false);
  const [isVisible, setIsVisible] = createSignal(props.isOpen);

  createEffect(() => {
    if (props.isOpen) {
      setIsVisible(true);
      setIsAnimating(true);
    } else {
      setIsAnimating(false);
      setTimeout(() => {
        setIsVisible(false);
      }, 1000); // Same duration as CSS transition
    }
  });

  const mockData = [
    {
      name: "Bitcoin",
      symbol: "BTC",
      amount: 0.00003,
      img: "gfx/bitcoin_placeholder.png",
      selected: false,
    },
    {
      name: "Ethereum",
      symbol: "ETH",
      amount: 0.0013,
      img: "gfx/ethereum_placeholder.png",
      selected: true,
    },
    {
      name: "Litecoin",
      symbol: "LTC",
      amount: 3.14,
      img: "gfx/litecoin_placeholder.png",
      selected: false,
    },
  ];

  return (
    <div class="fixed inset-0 z-50 flex items-end justify-center xl:grid-cols-3 xl:grid-rows-3">
      {/* Backdrop */}
      <div
        class={`fixed inset-0 bg-black opacity-50 ${
          isVisible() ? "transition-opacity duration-1000" : ""
        }`}
        onClick={props.onClose}
      ></div>

      {/* Modal */}
      <div
        class={`absolute w-full rounded-t-xl bg-r-light-background p-4 shadow-lg dark:bg-r-dark-foreground md:p-8 ${
          isVisible()
            ? "translate-y-0 transform transition-transform duration-1000"
            : "translate-y-full transform"
        } xl:col-start-2 xl:bottom-1/3 xl:w-1/3 xl:rounded-xl`}
      >
        {/* Modal content */}
        <h2 class="mb-4 text-lg font-semibold">Currencies</h2>
        <Currencies currencies={mockData} />
      </div>
    </div>
  );
}
