import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Fraction } from "./primitives/fraction";
import { Uuid } from "./primitives/uuid";

export const Order = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  user_id: Uuid,
  is_active: z.boolean(),
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  price: Fraction,
  quote_asset_volume: Fraction,
  quote_asset_volume_left: Fraction,
  maker_fee: Fraction,
});
export type Order = z.infer<typeof Order>;
