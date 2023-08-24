import Header from "~/components/Home/Header";
import { ContractAddressProvider } from "@packages/components/providers/ContractAddressProvider";
import WithdrawDashboard from "./Withdraw/WithdrawDashboard";

export default function Deposit() {
  return (
    <ContractAddressProvider>
      <div class="grid grid-rows-[128px_1fr] h-full">
        <div class="row-start-1 row-end-2">
          <Header text="Withdraw" />
        </div>
        <div class="row-start-2 row-end-3 relative overflow-clip">
          <div class="absolute inset-0 overflow-y-auto">
            <WithdrawDashboard />
          </div>
        </div>
      </div>
    </ContractAddressProvider>
  );
}
