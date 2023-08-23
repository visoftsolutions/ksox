import {
  Accessor,
  createContext,
  createSignal,
  JSX,
  onMount,
  useContext,
} from "solid-js";
import { z } from "zod";
import { Asset } from "@packages/types/asset";
import { Uuid } from "@packages/types/primitives/uuid";

const [assets, setAssets] = createSignal<Map<Uuid, Asset>>(new Map());
const AssetsContext = createContext<Accessor<Map<Uuid, Asset>>>(assets);
export function AssetsProvider(props: {
  children: JSX.Element;
  api_url: string;
}) {
  onMount(async () => {
    const assets = await fetch(`${props.api_url}/public/assets`)
      .then((r) => r.json())
      .then((r) => z.array(Asset).parse(r))
      .then((r) => {
        const map = new Map<Uuid, Asset>();
        r.forEach((asset) => map.set(asset.id, asset));
        return map;
      });
    setAssets(assets);
  });

  return (
    <AssetsContext.Provider value={assets}>
      {props.children}
    </AssetsContext.Provider>
  );
}
export function useAssets() {
  return useContext<Accessor<Map<Uuid, Asset>>>(AssetsContext);
}
