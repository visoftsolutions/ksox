import { Accessor, createContext, createEffect, createSignal, JSX, useContext } from "solid-js";
import { useParams } from "solid-start";
import { Asset } from "~/types/asset";
import { Uuid } from "~/types/primitives/uuid";
import { useAssets } from "./AssetsProvider";

export type Market = { quote_asset?: Asset; base_asset?: Asset };

const [market, setMarket] = createSignal<Market>({ quote_asset: undefined, base_asset: undefined });
const MarketContext = createContext<Accessor<Market>>(market);
export function MarketProvider(props: { children: JSX.Element }) {
  const params = useParams<{ baseAssetId?: Uuid; quoteAssetId?: Uuid }>();
  const assets = useAssets();

  createEffect(() => {
    console.log("MARKET");
    if (assets && params.baseAssetId && params.quoteAssetId) {
      setMarket({
        quote_asset: assets().get(params.quoteAssetId),
        base_asset: assets().get(params.baseAssetId),
      });
    }
  });

  return <MarketContext.Provider value={market}>{props.children}</MarketContext.Provider>;
}
export function useMarket() {
  return useContext<Accessor<Market>>(MarketContext);
}
