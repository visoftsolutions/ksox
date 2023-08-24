import { Index, batch, onCleanup, onMount } from "solid-js";
import ExternalTransferElement from "./ExternalTransferElement";
import { useSession } from "@packages/components/providers/SessionProvider";
import { usePrecision } from "@packages/components/providers/PrecisionProvider";
import subscribeEvents from "@packages/utils/subscribeEvents";
import { api } from "~/root";
import { createStore } from "solid-js/store";
import { Transfer } from "@packages/types/transfer";
import { z } from "zod";

export default function ExternalTransfers() {
  const session = useSession();
  const precision = usePrecision();

  const [externalTransfers, setExternalTransfers] = createStore<Transfer[]>([]);

  let eventsource: EventSource | undefined;
  onMount(async () => {
    const sessionValue = session();
    const precisionValue = precision();
    if (sessionValue && precisionValue) {
      eventsource = await subscribeEvents(
        `${api}/private/transfers/external`,
        "",
        "",
        (data) => {
          batch(() => {
            z.array(Transfer)
              .parse(data)
              .forEach((e) => {
                setExternalTransfers((state) => {
                  state.unshift(e);
                  return state;
                });
              });
          });
        }
      );
    }
  });
  onCleanup(() => {
    eventsource?.close();
  });

  return (
    <div class="rounded-xl bg-r-light-foreground dark:bg-r-dark-foreground scrollbar-thumb-r-dark-secondary-text dark:scrollbar-thumb-r-dark-active">
      <Index each={externalTransfers()}>
        {(element) => (
          <ExternalTransferElement
            name={element().name}
            img={element().img}
            date={element().date}
            text={element().text}
          />
        )}
      </Index>
    </div>
  );
}
