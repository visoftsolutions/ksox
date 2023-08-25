import CurrencyDisplay from "~/components/Home/CurrencyDisplay";
import NumberInput from "~/components/Inputs/NumberInput";
import { usePrecision } from "@packages/components/providers/PrecisionProvider";
import { useSelectedAsset } from "~/components/providers/SelectedAssetProvider";
import { Index, createSignal, onCleanup, onMount } from "solid-js";
import { Fraction, ev } from "@packages/types/primitives/fraction";
import ActionButton from "~/components/Atoms/Buttons/ActionButton";
import { handleWithdraw } from "@packages/utils/handlers/withdrawPermit";
import { useWallet } from "@packages/components/providers/WalletProvider";
import { useContractAddress } from "@packages/components/providers/ContractAddressProvider";
import TransferElement, { ITransferElement } from "../Home/TransferElement";
import { createStore } from "solid-js/store";
import { DisplayTransfer } from "@packages/types/transfer";
import firstLastChars from "@packages/utils/firstLastChars";
import subscribeEvents from "@packages/utils/subscribeEvents";
import params from "@packages/utils/params";
import { api } from "~/root";
import { z } from "zod";

export default function WithdrawDashboard() {
  const precision = usePrecision();
  const { selectedAsset } = useSelectedAsset();
  const [amount, setAmount] = createSignal(
    Fraction.parse({ numer: 0, denom: 1 }),
  );
  const wallet = useWallet();
  const [withdrawAddress, setWithdrawAddress] = createSignal(wallet.address!);
  const treasury_address = useContractAddress();
  const [transfers, setTransfers] = createStore<ITransferElement[]>([]);

  let eventsource: EventSource | undefined;

  const convertTransfer = (element: DisplayTransfer): ITransferElement => {
    return {
      name: element.user_name || firstLastChars(element.user_address, 4, 4),
      otherName:
        element.other_user_name || firstLastChars(element.other_user_id, 4, 4),
      amount: ev(element.amount),
      date: element.created_at,
      symbol: element.asset_symbol,
      direction: element.direction,
    };
  };

  onMount(async () => {
    eventsource = await subscribeEvents(
      `${api}/private/transfers/external`,
      params({ limit: 10, offset: 0 }),
      params({}),
      (data) => {
        setTransfers((state) =>
          z
            .array(DisplayTransfer)
            .parse(data)
            .map(convertTransfer)
            .concat(state),
        );
      },
    );
  });

  onCleanup(() => {
    eventsource?.close();
  });

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
            <Index each={transfers}>
              {(element) => (
                <TransferElement
                  name={element().name}
                  otherName={element().otherName}
                  date={element().date}
                  amount={element().amount}
                  symbol={element().symbol}
                  direction={element().direction}
                />
              )}
            </Index>
          </div>
        </div>
      </div>
    </div>
  );
}
