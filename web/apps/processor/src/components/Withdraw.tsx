import { ContractAddressProvider } from "@packages/components/providers/ContractAddressProvider";
import Header from "~/components/Home/Header";

export default function Withdraw() {
  return (
    <ContractAddressProvider>
      <div class="grid grid-rows-[128px_1fr] h-full">
        <div class="row-start-1 row-end-2">
          <Header />
        </div>
      </div>
    </ContractAddressProvider>
  );
}
