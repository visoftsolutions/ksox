import { Accessor, createContext, createSignal, JSX, onMount, useContext } from "solid-js";
import { z } from "zod";
import { api } from "~/root";
import { Asset } from "~/types/asset";
import { Uuid } from "~/types/primitives/uuid";

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
    // const map = new Map<Uuid, Asset>();
    // map.set("fe159f05-9b7d-4fc3-96f0-e105d2cde02d", {
    //   id: "fe159f05-9b7d-4fc3-96f0-e105d2cde02d",
    //   created_at: new Date(Date.now()),
    //   last_modification_at: new Date(Date.now()),
    //   name: "Bitcoin",
    //   symbol: "BTC",
    //   maker_fee: fFromBigint(0n),
    //   taker_fee: fFromBigint(0n),
    // });
    // map.set("fe159f05-9b7d-4fc3-96f0-e105d2cde02a", {
    //   id: "fe159f05-9b7d-4fc3-96f0-e105d2cde02a",
    //   created_at: new Date(Date.now()),
    //   last_modification_at: new Date(Date.now()),
    //   name: "Bitcoin",
    //   symbol: "BTC",
    //   maker_fee: fFromBigint(0n),
    //   taker_fee: fFromBigint(0n),
    // });
    setAssets(x);
  });

  return <AssetsContext.Provider value={assets}>{props.children}</AssetsContext.Provider>;
}
export function useAssets() {
  return useContext<Accessor<Map<Uuid, Asset>>>(AssetsContext);
}
