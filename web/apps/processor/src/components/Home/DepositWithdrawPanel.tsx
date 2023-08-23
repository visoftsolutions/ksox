import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export default function DepositWithdrawPanel() {
  return (
    <div class="text-center">
      <div class="inline-grid grid-cols-2 gap-6">
        <div class="grid grid-rows-[auto_auto] justify-items-center ">
          <div class="rounded-full p-3 aspect-square bg-r-blue-light-backdrop dark:bg-r-blue-dark-backdrop grid justify-center">
            <img
              src={joinPaths(base, "/gfx/deposit.svg")}
              width={24}
              height={24}
            />
          </div>
          <div class="text-xs font-semibold text-r-blue">deposit</div>
        </div>
        <div class="grid grid-rows-[auto_auto] justify-items-center ">
          <div class="rounded-full p-3 aspect-square bg-r-blue-light-backdrop dark:bg-r-blue-dark-backdrop grid justify-center">
            <img
              src={joinPaths(base, "/gfx/withdraw.svg")}
              width={24}
              height={24}
            />
          </div>
          <div class="text-xs font-semibold text-r-blue">withdraw</div>
        </div>
      </div>
    </div>
  );
}