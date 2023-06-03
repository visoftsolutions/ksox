import {
  BusinessDay,
  CandlestickSeriesPartialOptions,
  ChartOptions,
  ColorType,
  CrosshairMode,
  DeepPartial,
  HistogramSeriesPartialOptions,
  LineStyle,
  PriceScaleMode,
  UTCTimestamp,
} from "lightweight-charts";
import { Market } from "~/components/providers/MarketProvider";

export function chartOptions(makret?: Market): DeepPartial<ChartOptions> {
  const r: DeepPartial<ChartOptions> = {
    watermark: {
      color: "#7C7C8A15",
      visible: true,
      text: makret?.base_asset?.symbol + "/" + makret?.quote_asset?.symbol,
      fontSize: 100,
      fontFamily: "sans-serif",
      horzAlign: "center",
      vertAlign: "center",
    },
    layout: {
      background: {
        type: ColorType.Solid,
        color: "#1B1B23",
      },
      textColor: "#7C7C8A",
      fontSize: 11,
      fontFamily: "sans-serif",
    },
    rightPriceScale: {
      autoScale: true,
      mode: PriceScaleMode.Normal,
      invertScale: false,
      visible: true,
      textColor: "#7C7C8A",
      borderVisible: false,
    },
    timeScale: {
      rightOffset: 10,
      timeVisible: true,
      borderVisible: false,
    },
    crosshair: {
      mode: CrosshairMode.Normal,
      vertLine: {
        color: "#7C7C8A",
        style: LineStyle.Dashed,
        visible: true,
      },
      horzLine: {
        color: "#7C7C8A",
        style: LineStyle.Dashed,
        visible: true,
      },
    },
    grid: {
      vertLines: {
        color: "#7C7C8A",
        style: LineStyle.SparseDotted,
        visible: true,
      },
      horzLines: {
        color: "#7C7C8A",
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

export const candlestickOptions = {
  upColor: "#45B780",
  downColor: "#C73B3B",
  borderUpColor: "#45B780",
  borderDownColor: "#C73B3B",
  wickUpColor: "#45B780",
  wickDownColor: "#C73B3B",
} as CandlestickSeriesPartialOptions;

export const histogramOptions = {
  color: "#7C7C8A",
  lineWidth: 2,
  priceFormat: {
    type: "volume",
  },
  overlay: true,
  scaleMargins: {
    top: 0.6,
    bottom: 0,
  },
} as HistogramSeriesPartialOptions;
