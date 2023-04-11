import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Uuid } from "./primitives/uuid";
import { Volume } from "./primitives/volume";

export const Trade = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  taker_id: Uuid,
  order_id: Uuid,
  taker_quote_volume: Volume,
  taker_base_volume: Volume,
  maker_quote_volume: Volume,
  maker_base_volume: Volume,
});
export type Trade = z.infer<typeof Trade>;
