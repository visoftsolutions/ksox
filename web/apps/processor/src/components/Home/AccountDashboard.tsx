import CurrencyDisplay from "~/components/Home/CurrencyDisplay";
import DepositWithdrawPanel from "~/components/Home/DepositWithdrawPanel";
import { Index, createSignal, onCleanup, onMount } from "solid-js";
import TransferElement, { ITransferElement } from "../Atoms/TransferElement";
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
        name: element.user_name || firstLastChars(element.user_address, 4, 4),
        otherName:
          element.other_user_name ||
          firstLastChars(element.other_user_address, 4, 4),
        amount: ev(element.amount),
        date: element.created_at,
        symbol: element.asset_symbol,
        direction: element.direction,
        asset_icon_path: element.asset_icon_path,
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
              name={element().name}
              otherName={element().otherName}
              date={element().date}
              amount={element().amount}
              symbol={element().symbol}
              direction={element().direction}
              asset_icon_path={element().asset_icon_path}
            />
          )}
        </Index>
      </div>
    );
  };
}
