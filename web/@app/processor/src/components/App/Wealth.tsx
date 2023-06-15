import { Index, Show, batch, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import { Asset } from "@web/types/asset";
import { Uuid } from "@web/types/primitives/uuid";
import { AssetInfo } from "./Wealth/AssetInfo";
import { A } from "@solidjs/router";
import { SessionResponse } from "@web/components/providers/SessionProvider/models";

export default function CreateWealth(assets: Map<Uuid, Asset>, session: SessionResponse | undefined, precision: number) {
  return () => (
    <Show when={assets} fallback={<Wealth />}>
      <Wealth session={session} assets={assets} precision={precision} />
    </Show>
  );
}

export function Wealth(props: { session?: SessionResponse; assets?: Map<Uuid, Asset>; precision?: number }) {
  const [assetsStore, setAssetsStore] = createStore<{ [key: Uuid]: Asset }>({});

  onMount(async () => {
    batch(() => {
      props.assets?.forEach((e) => {
        setAssetsStore({ [e.id]: e });
      });
    });
  });

  return (
    <div class="grid grid-flow-row gap-2">
      <Index each={Object.values(assetsStore)}>
        {(element) => (
          <A href={`/asset/${element().id}`} class="cursor-pointer py-4">
            <AssetInfo asset={element()} precision={props.precision} session={props.session} />
          </A>
        )}
      </Index>
    </div>
  );
}
