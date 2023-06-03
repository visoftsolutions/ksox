import { Accessor, createContext, createEffect, createSignal, JSX, useContext } from "solid-js";
import { useParams } from "solid-start";
import { Asset } from "@web/types/asset";
import { Uuid } from "@web/types/primitives/uuid";
import { useAssets } from "./AssetsProvider";

const [asset, setAsset] = createSignal<Asset | undefined>(undefined);
const AssetContext = createContext<Accessor<Asset | undefined>>(asset);
export function AssetProvider(props: { children: JSX.Element }) {
  const params = useParams<{ assetId?: Uuid }>();
  const assets = useAssets();

  createEffect(() => {
    if (assets && params.assetId) {
      setAsset(assets().get(params.assetId));
    }
  });

  return <AssetContext.Provider value={asset}>{props.children}</AssetContext.Provider>;
}
export function useAsset() {
  return useContext<Accessor<Asset | undefined>>(AssetContext);
}
