import { format } from "numerable";
import { Show, createSignal, onCleanup, onMount } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { ValidateSignatureResponse } from "~/auth/mod";
import { api, base } from "~/root";
import { Asset } from "~/types/asset";
import { Fraction, ev } from "~/types/primitives/fraction";
import { Valut } from "~/types/valut";
import subscribeEvents from "~/utils/subscribeEvents";
import params from "~/utils/params";
import { formatTemplate } from "~/utils/precision";

export default function CreateAssetInfo(session?: ValidateSignatureResponse, asset?: Asset, precision?: number) {
  return () => (
    <Show when={asset && precision}>
      <AssetInfo asset={asset} precision={precision} session={session} />
    </Show>
  );
}

export function AssetInfo(props: { session?: ValidateSignatureResponse; asset?: Asset; precision?: number }) {
  const [balance, setBalance] = createSignal<Fraction | undefined>(undefined);

  let eventsource: EventSource | undefined;

  onMount(async () => {
    if (props.session && props.asset && props.precision) {
      const asset = props.asset;

      eventsource = await subscribeEvents(`${api}/private/balance`, params({ asset_id: asset.id }), params({ asset_id: asset.id }), (data) => {
        setBalance(Valut.parse(data).balance);
      });
    }
  });

  onCleanup(() => {
    eventsource?.close();
  });

  return (
    <div class="grid grid-cols-[50px_1fr] items-center justify-start overflow-clip">
      <div class="col-start-1 col-end-2 mr-2">
        <img src={joinPaths(base, "/gfx/asset_icons/" + props.asset?.symbol.toLowerCase() + ".svg")} width="40px" height="40px" />
      </div>
      <div class="col-start-2 col-end-3 grid grid-cols-[auto_1fr] items-center justify-center">
        <div class="col-start-1 col-end-2 text-ellipsis text-white">{`${props.asset?.name} (${props.asset?.symbol})`}</div>
        <div class="col-start-2 col-end-3 text-ellipsis text-right text-xl font-bold">
          {balance() != undefined ? format(ev(balance()!), formatTemplate(props.precision ?? 3)) : "---"}
        </div>
      </div>
    </div>
  );
}
