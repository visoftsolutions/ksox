import { Accessor, createContext, createEffect, createSignal, Index, JSX, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { joinPaths } from "solid-start/islands/server-router";
import { api, base } from "~/root";
import SearchInput from "~/components/Inputs/SearchInput";
import TriElement from "./TriElement/TriElement";
import TriElementHeader from "~/components/TriElement/TriElementHeader";
import params from "~/utils/params";
import z from "zod";
import { A } from "solid-start";

export const AssetResponse = z.object({
  id: z.string().uuid(),
  name: z.string(),
  symbol: z.string(),
});
export type AssetResponse = z.infer<typeof AssetResponse>;

export const AssetPairRecognitionResult = z.object({
  score: z.number(),
  asset0: AssetResponse,
  asset1: AssetResponse,
});
export type AssetPairRecognitionResult = z.infer<typeof AssetPairRecognitionResult>;

const fetchAssets = async (input: string) => {
  console.log(`fetching assets: ${api}/public/search`);
  return await fetch(`${api}/public/search?${params({ input })}`)
    .then((r) => r.json())
    .then((r) => z.array(AssetPairRecognitionResult).parse(r));
};

const [market, setMarket] = createSignal<{ quote_asset: AssetResponse; base_asset: AssetResponse } | null>(null);
const MarketContext = createContext<Accessor<{ quote_asset: AssetResponse; base_asset: AssetResponse } | null>>(market);
export function MarketProvider(props: { children: JSX.Element }) {
  return <MarketContext.Provider value={market}>{props.children}</MarketContext.Provider>;
}
export function useMarket() {
  return useContext<Accessor<{ quote_asset: AssetResponse; base_asset: AssetResponse } | null>>(MarketContext);
}

export default function Markets() {
  const [search, setSearch] = createSignal("");
  const [marketsState, setMarketsState] = createStore<Array<AssetPairRecognitionResult>>([]);

  createEffect(async () => {
    setMarketsState(await fetchAssets(search()));
  });

  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_1fr]">
      <div class="row-start-1 row-end-2">
        <div class="p-4 font-sanspro text-trades-label font-semibold">Markets</div>
        <div class="px-[12px]">
          <SearchInput
            class="mx-auto mb-2 w-full text-markets-searchbar"
            left={
              <>
                <img src={joinPaths(base, "gfx/search.svg")} />
              </>
            }
            onInput={(e) => {
              const value = (e.target as HTMLInputElement).value;
              setSearch(value);
            }}
          />
          <TriElementHeader
            column_0={<div class="text-left text-markets-sublabel">{"Market"}</div>}
            column_1={<div class="text-right text-markets-sublabel">{"Price"}</div>}
            column_2={<div class="text-right text-markets-sublabel">{"Change"}</div>}
          />
        </div>
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col overflow-scroll">
          <Index each={marketsState}>
            {(element, i) => (
              <A
                href={`/${element().asset0.id}/${element().asset1.id}`}
                onClick={() => setMarket({ quote_asset: element().asset1, base_asset: element().asset0 })}
              >
                <TriElement
                  class={`cursor-pointer select-none px-[12px] py-2 text-right font-sanspro text-markets-item ${i % 2 ? "bg-gray-3" : ""}`}
                  column_0={`${element().asset0.symbol}/${element().asset1.symbol}`}
                  column_1={0}
                  column_2={0}
                />
              </A>
            )}
          </Index>
        </div>
      </div>
    </div>
  );
}
