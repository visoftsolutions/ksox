import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Fraction } from "./primitives/fraction";
import { Uuid } from "./primitives/uuid";
import { Volume } from "./primitives/volume";

export enum CandlestickType {
  Interval,
  Tick,
}

export const Candlestick = z.object({
  id: Uuid,
  quoteAssetId: Uuid,
  baseAssetId: Uuid,
  kind: z.nativeEnum(CandlestickType),
  topen: Datetime,
  tclose: Datetime,
  open: Fraction,
  high: Fraction,
  low: Fraction,
  close: Fraction,
  span: z.number(),
  takerQuoteVolume: Volume,
  takerBaseVolume: Volume,
  makerQuoteVolume: Volume,
  makerBaseVolume: Volume,
});
export type Candlestick = z.infer<typeof Candlestick>;
