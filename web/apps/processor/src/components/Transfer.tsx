import Header from "~/components/Transfer/Header";
import { Index, createSignal } from "solid-js";
import TransferElement, { ITransferElement } from "./Transfer/TransferElement";

export default function Transfer() {
  const [transfers, setTransfers] = createSignal<ITransferElement[]>([]);

  return (
    <div class="grid grid-rows-[128px_1fr] h-full">
      <Header />
      <div class="relative">
        <div class="absolute inset-0 overflow-y-auto">
          <div class="grid grid-flow-row gap-4 ">
            <Index each={transfers()}>
              {(element) => (
                <TransferElement
                  user={element().user}
                  date={element().date}
                  amount={element().amount}
                  asset={element().asset}
                />
              )}
            </Index>
          </div>
        </div>
      </div>
    </div>
  );
}
