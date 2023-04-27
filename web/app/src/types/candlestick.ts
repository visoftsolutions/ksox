import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Fraction } from "./primitives/fraction";
import { Uuid } from "./primitives/uuid";

export enum CandlestickType {
  Interval = "Interval",
  Tick = "Tick",
}

export const Candlestick = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  kind: z.nativeEnum(CandlestickType),
  topen: Datetime,
  tclose: Datetime,
  open: Fraction,
  high: Fraction,
  low: Fraction,
  close: Fraction,
  span: z.number(),
  taker_quote_volume: Fraction,
  taker_base_volume: Fraction,
  maker_quote_volume: Fraction,
  maker_base_volume: Fraction,
});
export type Candlestick = z.infer<typeof Candlestick>;
