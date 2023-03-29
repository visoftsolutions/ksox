import { createEffect, Index } from "solid-js";
import { createStore } from "solid-js/store";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import SearchInput from "~/components/Inputs/SearchInput";
import TriElement, { TriElementComponent } from "./TriElement/TriElement";
import TriElementHeader from "~/components/TriElement/TriElementHeader";
import params from "~/utils/params";
import z from "zod";

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

export default function Markets() {
  const [marketsState, setMarketsState] = createStore<{ search: string; markets: Array<TriElementComponent> }>({
    search: "",
    markets: [],
  });

  createEffect(async () => {
    const BASE_URL = location.pathname;
    const API_URL = joinPaths(BASE_URL, "/api");
    const PUBLIC_URL = joinPaths(API_URL, "/public");

    await fetch(
      `${PUBLIC_URL}/search?${params({
        input: marketsState.search,
      })}`
    )
      .then((r) => r.json())
      .then((r) => z.array(AssetPairRecognitionResult).parse(r))
      .then((r) =>
        setMarketsState(
          "markets",
          r.map((el) => ({ column_0: `${el.asset0.symbol}/${el.asset1.symbol}`, column_1: 0, column_2: 0 }))
        )
      );
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
              const value: string = (e.target as HTMLInputElement).value;
              setMarketsState("search", value);
            }}
          />
          <TriElementHeader
            column_0={<div class="text-left text-markets-sublabel">{"Market"}</div>}
            column_1={<div class="text-right text-markets-sublabel">{"Price (USDT)"}</div>}
            column_2={<div class="text-right text-markets-sublabel">{"Change"}</div>}
          />
        </div>
      </div>
      <div class="relative row-start-2 row-end-3">
        <div class="absolute top-0 left-0 right-0 bottom-0 flex flex-col overflow-scroll">
          <Index each={marketsState.markets}>
            {(element, i) => (
              <TriElement
                class={`cursor-pointer select-none px-[12px] py-2 text-right font-sanspro text-markets-item ${i % 2 ? "bg-gray-3" : ""}`}
                onClick={() => {
                  console.log("clicked");
                }}
                column_0={element().column_0}
                column_1={element().column_1}
                column_2={element().column_2}
              />
            )}
          </Index>
        </div>
      </div>
    </div>
  );
}
