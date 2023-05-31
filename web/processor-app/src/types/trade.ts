import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Fraction } from "./primitives/fraction";
import { Direction } from "./primitives/direction";
import { Uuid } from "./primitives/uuid";

export const PublicTrade = z.object({
  price: Fraction,
  volume: Fraction,
  time: Datetime,
  direction: Direction,
});
export type PublicTrade = z.infer<typeof PublicTrade>;

export const PrivateTrade = z.object({
  id: Uuid,
  created_at: Datetime,
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  price: Fraction,
  taker_quote_volume: Fraction,
  taker_base_volume: Fraction,
  maker_quote_volume: Fraction,
  maker_base_volume: Fraction,
  direction: Direction,
});
export type PrivateTrade = z.infer<typeof PrivateTrade>;
