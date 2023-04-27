import { Show, onCleanup, onMount } from "solid-js";
import { Market } from "~/utils/providers/MarketProvider";
import { CandlestickChart } from "./Chart/candlestickChart";
import { chartOptions, histogramOptions, candlestickOptions } from "./Chart/config";
import { api } from "~/root";
import params from "~/utils/params";
import { Candlestick } from "~/types/candlestick";
import { z } from "zod";

export default function CreateChart(market: Market) {
  return () => (
    <Show when={market && market.quote_asset && market.base_asset} fallback={<Chart />}>
      <Chart market={market} />;
    </Show>
  );
}

enum ChartType {
  Interval = "Interval",
}

export function Chart(props: { market?: Market }) {
  let ChartDOM!: HTMLDivElement;

  let events: EventSource | null = null;

  onMount(async () => {
    if (props.market && props.market?.quote_asset && props.market?.base_asset) {
      const market = props.market;
      const quote_asset = props.market.quote_asset;
      const basee_asset = props.market.base_asset;
      const chart = new CandlestickChart(ChartDOM as HTMLElement, chartOptions(market), histogramOptions, candlestickOptions);
      const interval = 60000;
      const now = Date.now();
      let reference_point = now - (now % interval);

      events = new EventSource(
        `${api}/public/ohlcv/sse?${params({
          quote_asset_id: quote_asset.id,
          base_asset_id: basee_asset.id,
          kind: ChartType.Interval.toString(),
          reference_point: new Date(reference_point).toISOString(),
          span: 60000000,
        })}`
      );
      events.onopen = async () => {
        for (let i = 0; i < 5; i++) {
          reference_point -= interval;
          await fetch(
            `${api}/public/ohlcv?${params({
              quote_asset_id: quote_asset.id,
              base_asset_id: basee_asset.id,
              kind: ChartType.Interval.toString(),
              reference_point: new Date(reference_point).toISOString(),
              span: 60000000,
            })}`
          )
            .then((r) => r.json())
            .then((r) => z.nullable(Candlestick).parse(r))
            .then((r) => {
              if (r != null) chart.unshift(r);
            });
        }
      };
      events.onmessage = (ev) => {
        chart.push(Candlestick.parse(JSON.parse(ev.data)));
      };
    }
  });

  onCleanup(() => {
    events?.close();
  });

  return <div ref={ChartDOM} class="absolute bottom-0 left-0 right-0 top-0 bg-gray-2" />;
}
