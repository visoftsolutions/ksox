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
  taker_quote_volume: Volume,
  taker_base_volume: Volume,
  maker_quote_volume: Volume,
  maker_base_volume: Volume,
});
export type Candlestick = z.infer<typeof Candlestick>;
