import CurrencyDisplay from "~/components/Home/CurrencyDisplay";
import NumberInput from "~/components/Inputs/NumberInput";
import { usePrecision } from "@packages/components/providers/PrecisionProvider";
import { useSelectedAsset } from "~/components/providers/SelectedAssetProvider";
import { createSignal } from "solid-js";
import { Fraction } from "@packages/types/primitives/fraction";
import ActionButton from "~/components/Atoms/Buttons/ActionButton";
import { handleDeposit } from "@packages/utils/handlers/depositPermit";
import { useWallet } from "@packages/components/providers/WalletProvider";
import { useContractAddress } from "@packages/components/providers/ContractAddressProvider";

export default function DepositDashboard() {
  const precision = usePrecision();
  const { selectedAsset } = useSelectedAsset();
  const [amount, setAmount] = createSignal(
    Fraction.parse({ numer: 0, denom: 1 }),
  );
  const wallet = useWallet();
  const treasury_address = useContractAddress();

  return (
    <div class="rounded-xl bg-r-light-foreground dark:bg-r-dark-foreground scrollbar-thumb-r-dark-secondary-text dark:scrollbar-thumb-r-dark-active">
      <CurrencyDisplay />
      <div class="h-4" />
      <div class="grid">
        <NumberInput
          class="p-1 text-submit-label justify-self-center"
          precision={precision()}
          left={"Quantity"}
          right={selectedAsset()?.symbol}
          value={amount()}
          onChange={(f) => {
            setAmount(f);
          }}
        />
        <div class="h-4" />
        <ActionButton
          text="Deposit"
          onClick={async () => {
            const asset = selectedAsset();
            const treasury = treasury_address!();
            if (asset && treasury) {
              await handleDeposit({
                asset: asset,
                amount: amount(),
                wallet,
                treasury_address: treasury,
              });
            }
          }}
        />
        <div class="h-4" />
      </div>

      <div>
        <p class="text-sans mx-5 text-sm text-bold text-r-dark-secondary-text">
          Deposits
        </p>
        {/* <DepositTransactions /> */}
      </div>
    </div>
  );
}
