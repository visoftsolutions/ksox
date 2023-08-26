import { createEffect, createSignal } from "solid-js";
import Header from "~/components/Header";
import UserDasboard from "~/components/Transfer/UserDashboard";
import TransferToDashboard from "./TransferTo/TransferToDashboard";

export default function TransferTo() {
  const [search, setSearch] = createSignal<string>();

  return (
    <div class="grid grid-rows-[128px_1fr] h-full">
      <div class="row-start-1 row-end-2">
        <Header text="Transfer" setSearch={setSearch} />
      </div>
      <div class="row-start-2 row-end-3 relative overflow-clip p-8 bg-r-light-foreground dark:bg-r-dark-foreground rounded-xl">
        <TransferToDashboard search={search()} />
      </div>
    </div>
  );
}
