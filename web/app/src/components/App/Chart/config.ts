import {
  BusinessDay,
  ChartOptions,
  ColorType,
  CrosshairMode,
  DeepPartial,
  HistogramSeriesPartialOptions,
  LineStyle,
  PriceScaleMode,
  UTCTimestamp,
} from "lightweight-charts";
import { Market } from "~/utils/providers/MarketProvider";

export function chartOptions(makret?: Market): DeepPartial<ChartOptions> {
  const r: DeepPartial<ChartOptions> = {
    watermark: {
      color: "#FFFFFF08",
      visible: true,
      text: makret?.base_asset?.symbol + "/" + makret?.quote_asset?.symbol,
      fontSize: 100,
    },
    layout: {
      background: {
        type: ColorType.Solid,
        color: "#111118",
      },
      textColor: "#eeeeee",
      fontSize: 11,
    },
    rightPriceScale: {
      autoScale: true,
      mode: PriceScaleMode.Normal,
      invertScale: false,
      visible: true,
    },
    timeScale: {
      rightOffset: 10,
      timeVisible: true,
    },
    crosshair: {
      mode: CrosshairMode.Normal,
      vertLine: {
        color: "#FFFFFF44",
        style: LineStyle.Dashed,
        visible: true,
      },
      horzLine: {
        color: "#FFFFFF44",
        style: LineStyle.Dashed,
        visible: true,
      },
    },
    grid: {
      vertLines: {
        color: "#FFFFFF22",
        style: LineStyle.SparseDotted,
        visible: true,
      },
      horzLines: {
        color: "#FFFFFF22",
        style: LineStyle.SparseDotted,
        visible: true,
      },
    },
    localization: {
      timeFormatter: (time: UTCTimestamp | BusinessDay) => {
        return new Intl.DateTimeFormat("en-GB", { dateStyle: "medium", timeStyle: "medium" }).format((time as UTCTimestamp) * 1000);
      },
    },
  };
  return r;
}

export const histogramOptions = {
  color: "#FFFFFF22",
  priceFormat: {
    type: "volume",
  },
  overlay: true,
  scaleMargins: {
    top: 0.8,
    bottom: 0,
  },
} as HistogramSeriesPartialOptions;
