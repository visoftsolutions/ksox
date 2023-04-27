import { createEffect, createSignal, Index } from "solid-js";
import { createStore } from "solid-js/store";
import { joinPaths } from "solid-start/islands/server-router";
import { api, base } from "~/root";
import SearchInput from "~/components/Inputs/SearchInput";
import TriElement from "./TriElement/TriElement";
import TriElementHeader from "~/components/App/TriElement/TriElementHeader";
import params from "~/utils/params";
import z from "zod";
import { A } from "solid-start";
import NavButton from "./Buttons/NavButton";

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

enum Tab {
  Markets,
  Favorites,
}

const querySearch = async (input: string) => {
  return await fetch(`${api}/public/search?${params({ input })}`)
    .then((r) => r.json())
    .then((r) => z.array(AssetPairRecognitionResult).parse(r));
};

export default function Markets() {
  const [search, setSearch] = createSignal("");
  const [marketsState, setMarketsState] = createStore<Array<AssetPairRecognitionResult>>([]);
  const [tab, setTab] = createSignal(Tab.Markets);

  createEffect(async () => {
    setMarketsState(await querySearch(search()));
  });

  return (
    <div class="grid h-full grid-cols-1 grid-rows-[auto_76px_1fr]">
      <div class="row-start-1 row-end-2 grid items-start justify-start gap-[1px]">
        <NavButton
          class="col-start-1 col-end-2"
          highlighted={tab() == Tab.Markets}
          onClick={() => {
            setTab(Tab.Markets);
          }}
        >
          <span class="text-navButton">Markets</span>
        </NavButton>
        <NavButton
          class="col-start-2 col-end-3"
          highlighted={tab() == Tab.Favorites}
          onClick={() => {
            setTab(Tab.Favorites);
          }}
        >
          <div class="grid items-center justify-center gap-1 text-navButton">
            <img src={joinPaths(base, "/gfx/star.svg")} alt="star" width="16px" height="16px" class="col-start-1 col-end-2" />
            <div class="col-start-2 col-end-3">Favorites</div>
          </div>
        </NavButton>
      </div>
      <div class="row-start-2 row-end-3 grid items-center px-3">
        <SearchInput
          class="text-markets-searchbar row-start-1 row-end-2 mx-auto mt-3 w-full"
          left={
            <>
              <img src={joinPaths(base, "/gfx/search.svg")} alt="search" width="20px" />
            </>
          }
          onInput={(e) => {
            const value = (e.target as HTMLInputElement).value;
            setSearch(value);
          }}
        />
        <TriElementHeader class="row-start-2 row-end-3 self-end" column_0={"Market"} column_1={"Price"} column_2={"Change"} />
      </div>
      <div class="relative row-start-3 row-end-4">
        <div class="absolute bottom-0 left-0 right-0 top-0 flex flex-col overflow-y-auto">
          <Index each={marketsState}>
            {(element, i) => (
              <A href={`/${element().asset0.id}/${element().asset1.id}`}>
                <TriElement
                  class={`cursor-pointer select-none px-3 py-2 ${i % 2 && "bg-gray-3"}`}
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
