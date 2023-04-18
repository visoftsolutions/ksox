import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Uuid } from "./primitives/uuid";
import { Fraction } from "./primitives/fraction";

export const Trade = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  taker_id: Uuid,
  order_id: Uuid,
  price: Fraction,
  taker_quote_volume: Fraction,
  taker_base_volume: Fraction,
  maker_quote_volume: Fraction,
  maker_base_volume: Fraction,
});
export type Trade = z.infer<typeof Trade>;
