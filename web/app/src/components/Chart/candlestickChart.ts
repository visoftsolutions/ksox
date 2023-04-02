import { ChartOptions, createChart, DeepPartial, HistogramSeriesPartialOptions, ISeriesApi, UTCTimestamp } from "lightweight-charts";
import { TOHLC, TV } from "./models";
import { Candlestick } from "~/types/candlestick";
import { evaluateInv } from "~/types/primitives/fraction";
import { fromWei } from "~/utils/converters/wei";

export class CandlestickChart {
  private tohlc_data: TOHLC[];
  private tv_data: TV[];
  private candlestickSeries: ISeriesApi<"Candlestick">;
  private histogramSeries: ISeriesApi<"Histogram">;
  constructor(container: HTMLElement, chartOptions: DeepPartial<ChartOptions>, histogramOptions: HistogramSeriesPartialOptions) {
    this.tohlc_data = [];
    this.tv_data = [];
    const chart = createChart(container, chartOptions);
    new ResizeObserver((entries) => {
      const newRect = entries[0].contentRect;
      chart.applyOptions({
        width: newRect.width,
        height: newRect.height,
      });
    }).observe(container as Element);

    this.histogramSeries = chart.addHistogramSeries(histogramOptions);
    this.candlestickSeries = chart.addCandlestickSeries();
    chart.timeScale().fitContent();
  }

  extractTOHLC(candlestick: Candlestick): TOHLC {
    return {
      time: Math.floor(candlestick.topen.getTime() / 1000) as UTCTimestamp,
      open: evaluateInv(candlestick.open),
      high: evaluateInv(candlestick.high),
      low: evaluateInv(candlestick.low),
      close: evaluateInv(candlestick.close),
    };
  }

  extractTV(candlestick: Candlestick): TV {
    return {
      time: Math.floor(candlestick.topen.getTime() / 1000) as UTCTimestamp,
      volume: fromWei(candlestick.taker_quote_volume),
    };
  }

  unshift(candlestick: Candlestick) {
    const tohlc: TOHLC = this.extractTOHLC(candlestick);
    const tv: TV = this.extractTV(candlestick);
    this.tohlc_data.unshift(tohlc);
    this.tv_data.unshift(tv);
    this.candlestickSeries.update(tohlc);
    this.histogramSeries.update(tv);
  };

  push(candlestick: Candlestick) {
    const tohlc: TOHLC = this.extractTOHLC(candlestick);
    const tv: TV = this.extractTV(candlestick);
    this.tohlc_data.push(tohlc);
    this.tv_data.push(tv);
    this.candlestickSeries.setData(this.tohlc_data);
    this.histogramSeries.setData(this.tv_data);
  };
}
