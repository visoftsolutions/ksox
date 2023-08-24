import CurrencyDisplay from "~/components/Home/CurrencyDisplay";
import DepositWithdrawPanel from "~/components/Home/DepositWithdrawPanel";
import { useAssets } from "@packages/components/providers/AssetsProvider";
import { createMemo } from "solid-js";
import { ITransfer } from "./Transfer";
import Transfers from "./Transfers";
import { User } from "@packages/types/user";

export default function AccountDashboard() {
  const assets = useAssets();
  const assetsList = createMemo(() => [...assets().values()]);

  return (
    <div class="rounded-xl bg-r-light-foreground dark:bg-r-dark-foreground scrollbar-thumb-r-dark-secondary-text dark:scrollbar-thumb-r-dark-active">
      <CurrencyDisplay />
      <DepositWithdrawPanel />
      <div>
        <p class="text-sans mx-5 text-sm text-bold text-r-dark-secondary-text">
          Transfers
        </p>
        <Transfers transfers={[]} />
      </div>
    </div>
  );
}
