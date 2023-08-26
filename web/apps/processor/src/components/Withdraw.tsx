import Header from "~/components/Header";
import { ContractAddressProvider } from "@packages/components/providers/ContractAddressProvider";
import WithdrawDashboard from "~/components/ExternalTransfers/WithdrawDashboard";
import { createSignal } from "solid-js";

export default function Deposit() {
  const [search, setSearch] = createSignal<string>();

  return (
    <ContractAddressProvider>
      <div class="grid grid-rows-[128px_1fr] h-full">
        <div class="row-start-1 row-end-2">
          <Header text="Withdraw" setSearch={setSearch} />
        </div>
        <div class="row-start-2 row-end-3 relative overflow-clip p-8 bg-r-light-foreground dark:bg-r-dark-foreground rounded-xl">
          <WithdrawDashboard />
        </div>
      </div>
    </ContractAddressProvider>
  );
}
