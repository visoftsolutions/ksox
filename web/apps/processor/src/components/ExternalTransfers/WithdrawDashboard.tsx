import CurrencyDisplay from "~/components/Home/CurrencyDisplay";
import NumberInput from "~/components/Inputs/NumberInput";
import { usePrecision } from "@packages/components/providers/PrecisionProvider";
import { useSelectedAsset } from "~/components/providers/SelectedAssetProvider";
import { Index, createSignal } from "solid-js";
import { Fraction } from "@packages/types/primitives/fraction";
import ActionButton from "~/components/Atoms/Buttons/ActionButton";
import { handleWithdraw } from "@packages/utils/handlers/withdrawPermit";
import { useWallet } from "@packages/components/providers/WalletProvider";
import { useContractAddress } from "@packages/components/providers/ContractAddressProvider";
import TransferElement, { ITransferElement } from "../Home/TransferElement";

export default function WithdrawDashboard() {
  const precision = usePrecision();
  const { selectedAsset } = useSelectedAsset();
  const [amount, setAmount] = createSignal(
    Fraction.parse({ numer: 0, denom: 1 }),
  );
  const wallet = useWallet();
  const [withdrawAddress, setWithdrawAddress] = createSignal(wallet.address!);
  const treasury_address = useContractAddress();
  const [transfers, setTransfers] = createSignal<ITransferElement[]>([]);

  return (
    <div class="grid grid-rows-[auto_auto_auto_1fr] h-full gap-4">
      <CurrencyDisplay />
      <div class="grid grid-cols-[auto_auto] gap-5">
        <NumberInput
          class="w-full p-1 border text-md justify-self-center"
          precision={precision()}
          left={"Quantity"}
          right={selectedAsset()?.symbol}
          value={amount()}
          onChange={(f) => {
            setAmount(f);
          }}
        />
        <ActionButton
          text="Withdraw"
          onClick={async () => {
            const asset = selectedAsset();
            const treasury = treasury_address!();
            if (asset && treasury) {
              await handleWithdraw({
                address_value: withdrawAddress(),
                asset: asset,
                amount: amount(),
                wallet,
                treasury_address: treasury,
              });
            }
          }}
        />
      </div>
      <p class="text-sans text-sm text-bold text-r-dark-secondary-text">
        Withdraws
      </p>
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
