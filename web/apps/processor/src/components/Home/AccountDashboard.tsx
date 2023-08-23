import CurrencyDisplay from "~/components/Home/CurrencyDisplay";
import DepositWithdrawPanel from "~/components/Home/DepositWithdrawPanel";
import { useAssets } from "@packages/components/providers/AssetsProvider";
import { Index, createMemo, createSignal } from "solid-js";
import TransferElement, { ITransferElement } from "./TransferElement";
import { User } from "@packages/types/user";
import { Asset } from "@packages/types/asset";

export default function AccountDashboard() {
  const assets = useAssets();
  const assetsList = createMemo(() => [...assets().values()]);
  const [transfers, setTransfers] = createSignal<ITransferElement[]>([]);

  const user1: User = {
    id: "79253ba2-b737-4177-ad6a-3f1c477a0654",
    address: "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
    created_at: new Date(Date.now()),
    last_modification_at: new Date(Date.now()),
    email: null,
    name: null,
    phone: null,
  };

  const asset1: Asset = {
    id: "8728ad10-1100-4cf3-b48f-73c2f5bc559a",
    created_at: new Date(Date.now()),
    last_modification_at: new Date(Date.now()),
    name: "Wrapped Bitcoin",
    symbol: "BTC",
    icon_path: "/gfx/asset_icons/btc.svg",
    address: "0x5fbdb2315678afecb367f032d93f642f64180aa3",
    decimals: {
      numer: 1000000000000000000n,
      denom: 1n,
    },
    maker_fee: {
      numer: 1000000000000000000n,
      denom: 1n,
    },
    taker_fee: {
      numer: 1000000000000000000n,
      denom: 1n,
    },
    transfer_fee: {
      numer: 1000000000000000000n,
      denom: 1n,
    },
  };

  setTransfers([
    {
      user: user1,
      date: new Date(Date.now()),
      amount: 0.1012,
      asset: asset1,
    },
    {
      user: user1,
      date: new Date(Date.now()),
      amount: 0.1012,
      asset: asset1,
    },
  ]);

  return (
    <div class="grid grid-rows-[auto_auto_auto_1fr] h-full gap-4">
      <CurrencyDisplay />
      <DepositWithdrawPanel />
      <p class="text-sans text-sm text-bold text-r-dark-secondary-text">
        Transfers
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
