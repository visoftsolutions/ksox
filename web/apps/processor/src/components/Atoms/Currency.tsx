import Picture from "~/components/Atoms/Picture";
import { Asset } from "@packages/types/asset";
import { api } from "~/root";
import { useSession } from "@packages/components/providers/SessionProvider";
import { usePrecision } from "@packages/components/providers/PrecisionProvider";
import { createSignal, onCleanup, onMount } from "solid-js";
import params from "@packages/utils/params";
import { Fraction, ev } from "@packages/types/primitives/fraction";
import { Valut } from "@packages/types/valut";
import subscribeEvents from "@packages/utils/subscribeEvents";
import { format } from "numerable";
import { formatTemplate } from "@packages/utils/precision";

export interface ICurrency {
  asset: Asset;
  selected: boolean;
  onClick?: () => void;
}

export default function Currency(props: ICurrency) {
  const session = useSession();
  const precision = usePrecision();
  const [balance, setBalance] = createSignal<Fraction | null>(null);

  let eventsource: EventSource | undefined;

  onMount(async () => {
    if (session() && props.asset && precision()) {
      eventsource = await subscribeEvents(
        `${api}/private/balance`,
        params({ asset_id: props.asset.id }),
        params({ asset_id: props.asset.id }),
        (data) => {
          setBalance(Fraction.parse(Valut.parse(data).balance.Finite));
        },
      );
    }
  });

  onCleanup(() => {
    eventsource?.close();
  });

  return (
    <div
      class={`rounded-xl ${
        props.selected
          ? "bg-r-light-modal-selected dark:bg-r-dark-modal-selected"
          : "bg-r-light-foreground dark:bg-r-dark-modal-foreground"
      } cursor-pointer`}
      onClick={() => {
        if (props.onClick) {
          props.onClick();
        }
      }}
    >
      <div class="flex justify-between">
        <div class="m-4 flex">
          <Picture src={props.asset.icon_path} alt="test" size={42} />
          <div class="ml-4">
            <p class="text-r-light-text dark:text-r-dark-text font-sans font-bold">
              {props.asset.name}
            </p>
            <p class="font-sans text-xs text-r-dark-secondary-text">
              {props.asset.symbol}
            </p>
          </div>
        </div>
        <div class="m-4 flex flex-col items-end">
          <p class="text-r-light-text dark:text-r-dark-text font-sans ">
            {balance() ? format(ev(balance()!), formatTemplate(3)) : "---"}
          </p>
        </div>
      </div>
    </div>
  );
}
