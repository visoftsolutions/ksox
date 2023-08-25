import CurrencyDisplay from "~/components/Home/CurrencyDisplay";
import DepositWithdrawPanel from "~/components/Home/DepositWithdrawPanel";
import { useAssets } from "@packages/components/providers/AssetsProvider";
import {
  Index,
  batch,
  createMemo,
  createSignal,
  onCleanup,
  onMount,
} from "solid-js";
import TransferElement, { ITransferElement } from "./TransferElement";
import { User } from "@packages/types/user";
import { Asset } from "@packages/types/asset";
import subscribeEvents from "@packages/utils/subscribeEvents";
import { api } from "~/root";
import params from "@packages/utils/params";
import { z } from "zod";
import { DisplayTransfer } from "@packages/types/transfer";
import firstLastChars from "@packages/utils/firstLastChars";
import { ev } from "@packages/types/primitives/fraction";
import { Dynamic } from "solid-js/web";
import { SessionResponse } from "@packages/components/providers/SessionProvider/models";
import { session } from "@packages/components/providers/SessionProvider";

export default function AccountDashboard() {
  return (
    <div class="grid grid-rows-[auto_auto_auto_1fr] h-full gap-4">
      <CurrencyDisplay />
      <DepositWithdrawPanel />
      <p class="text-sans text-sm text-bold text-r-dark-secondary-text">
        Transfers
      </p>
      <div class="relative">
        <div class="absolute inset-0 overflow-y-auto">
          <Dynamic component={CreateTransfers(session())} />
        </div>
      </div>
    </div>
  );
}

export function CreateTransfers(session: SessionResponse | undefined) {
  return () => {
    const [transfers, setTransfers] = createSignal<ITransferElement[]>([]);

    let eventsource: EventSource | undefined;

    const convertTransfer = (element: DisplayTransfer): ITransferElement => {
      return {
        from:
          element.from_user_name ||
          firstLastChars(element.from_user_address, 4, 4),
        to:
          element.to_user_name || firstLastChars(element.to_user_address, 4, 4),
        amount: ev(element.amount),
        date: element.created_at,
        symbol: element.asset_symbol,
      };
    };

    onMount(async () => {
      if (session) {
        eventsource = await subscribeEvents(
          `${api}/private/transfers/display`,
          params({ limit: 10, offset: 0 }),
          params({}),
          (data) => {
            setTransfers([
              ...z.array(DisplayTransfer).parse(data).map(convertTransfer),
              ...transfers(),
            ]);
          },
        );
      }
    });

    onCleanup(() => {
      eventsource?.close();
    });

    return (
      <div class="grid grid-flow-row gap-4 ">
        <Index each={transfers()}>
          {(element) => (
            <TransferElement
              from={element().from}
              to={element().to}
              date={element().date}
              amount={element().amount}
              symbol={element().symbol}
            />
          )}
        </Index>
      </div>
    );
  };
}
