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

export default function CreateAssetInfo(asset?: Asset, session?: ValidateSignatureResponse, precision?: number) {
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
    <div class="grid grid-rows-[auto_auto_auto] items-center gap-3">
      <div class="row-start-1 row-end-2">
        <img src={joinPaths(base, "/gfx/asset_icons/" + props.asset?.symbol.toLowerCase() + ".svg")} width="60px" height="60px" class="m-auto" />
      </div>
      <div class="row-start-2 row-end-3 grid grid-rows-2 items-center justify-center gap-1 text-center font-lexend text-lg font-bold">
        <div class="">{props.asset?.name}</div>
        <div class="">{props.asset?.symbol}</div>
      </div>
      <div class="row-start-3 row-end-4 text-center font-lexend text-2xl font-bold">
        {balance() != undefined ? format(ev(balance()!), formatTemplate(props.precision ?? 3)) : "---"}
      </div>
    </div>
  );
}
