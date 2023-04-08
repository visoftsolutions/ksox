import { format } from "numerable";
import { Show, createSignal, onCleanup, onMount } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { ValidateSignatureResponse } from "~/auth/mod";
import { api, base } from "~/root";
import { Asset } from "~/types/asset";
import { Valut } from "~/types/valut";
import { fromWei } from "~/utils/converters/wei";
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
  const [balance, setBalance] = createSignal<bigint | undefined>(undefined);

  let events: EventSource | null = null;

  onMount(() => {
    if (props.session && props.asset && props.precision) {
      const asset = props.asset;

      events = new EventSource(
        `${api}/private/balance/sse?${params({
          asset_id: asset.id,
        })}`,
        { withCredentials: true }
      );
      events.onopen = async () => {
        await fetch(
          `${api}/private/balance?${params({
            asset_id: asset.id,
          })}`,
          {
            method: "GET",
            credentials: "same-origin",
          }
        )
          .then((r) => r.json())
          .then((r) => Valut.parse(r))
          .then((r) => setBalance(r.balance));
      };
      events.onmessage = (ev) => {
        setBalance(Valut.parse(JSON.parse(ev.data)).balance);
      };
    }
  });

  onCleanup(() => {
    events?.close();
  });

  return (
    <div class="row-start-1 row-end-2 grid grid-cols-[36px_1fr] items-center justify-start overflow-clip px-4">
      <div class="col-start-1 col-end-2 mr-2">
        <img src={joinPaths(base, "/gfx/asset_icons/" + props.asset?.symbol.toLowerCase() + ".svg")} width="28px" height="28px" />
      </div>
      <div class="col-start-2 col-end-3 grid">
        <div class="row-start-1 row-end-2 text-ellipsis text-white">{`${props.asset?.name} (${props.asset?.symbol})`}</div>
        <div class="row-start-2 row-end-3 text-ellipsis text-orderbook-item">
          {balance() != undefined ? format(fromWei(balance()!), formatTemplate(props.precision ?? 3)) : "---"}
        </div>
      </div>
    </div>
  );
}
