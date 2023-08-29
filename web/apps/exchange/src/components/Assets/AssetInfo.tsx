import { format } from "numerable";
import { Show, createSignal, onCleanup, onMount } from "solid-js";
import { api } from "~/root";
import { Asset } from "@packages/types/asset";
import { Fraction, ev } from "@packages/types/primitives/fraction";
import { Valut } from "@packages/types/valut";
import subscribeEvents from "@packages/utils/subscribeEvents";
import params from "@packages/utils/params";
import { formatTemplate } from "@packages/utils/precision";
import { SessionResponse } from "@packages/components/providers/SessionProvider/models";

export default function CreateAssetInfo(
  session?: SessionResponse,
  asset?: Asset,
  precision?: number,
) {
  return () => (
    <Show when={asset && precision}>
      <AssetInfo asset={asset} precision={precision} session={session} />
    </Show>
  );
}

export function AssetInfo(props: {
  session?: SessionResponse;
  asset?: Asset;
  precision?: number;
}) {
  const [balance, setBalance] = createSignal<Fraction | undefined>(undefined);

  let eventsource: EventSource | undefined;

  onMount(async () => {
    if (props.session && props.asset && props.precision) {
      const asset = props.asset;

      eventsource = await subscribeEvents(
        `${api}/private/balance`,
        params({ asset_id: asset.id }),
        params({ asset_id: asset.id }),
        (data) => {
          setBalance(Valut.parse(data).balance.Finite);
        },
      );
    }
  });

  onCleanup(() => {
    eventsource?.close();
  });

  return (
    <div class="row-start-1 row-end-2 grid grid-cols-[36px_1fr] items-center justify-start overflow-clip px-4">
      <div class="col-start-1 col-end-2 mr-2">
        <img
          src={props.asset?.icon_path}
          width="28px"
          height="28px"
        />
      </div>
      <div class="col-start-2 col-end-3 grid">
        <div class="row-start-1 row-end-2 text-ellipsis text-white">{`${props.asset?.name} (${props.asset?.symbol})`}</div>
        <div class="row-start-2 row-end-3 text-ellipsis text-orderbook-item">
          {balance()
            ? format(ev(balance()!), formatTemplate(3))
            : "---"}
        </div>
      </div>
    </div>
  );
}
