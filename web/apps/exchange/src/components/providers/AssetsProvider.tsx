import {
  Accessor,
  createContext,
  createSignal,
  JSX,
  onMount,
  useContext,
} from "solid-js";
import { z } from "zod";
import { api } from "~/root";
import { Asset } from "@packages/types/asset";
import { Uuid } from "@packages/types/primitives/uuid";

const fetchAssets = async () => {
  return await fetch(`${api}/public/assets`)
    .then((r) => r.json())
    .then((r) => z.array(Asset).parse(r))
    .then((r) => {
      const map = new Map<Uuid, Asset>();
      r.forEach((asset) => map.set(asset.id, asset));
      return map;
    });
};

const [assets, setAssets] = createSignal<Map<Uuid, Asset>>(new Map());
const AssetsContext = createContext<Accessor<Map<Uuid, Asset>>>(assets);
export function AssetsProvider(props: { children: JSX.Element }) {
  onMount(async () => {
    const x = await fetchAssets();
    setAssets(x);
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
