import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Fraction } from "./primitives/fraction";
import { Uuid } from "./primitives/uuid";
import { Volume } from "./primitives/volume";

export const Order = z.object({
  id: Uuid,
  created_at: Datetime,
  user_id: Uuid,
  is_active: z.boolean(),
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  quote_asset_volume: Volume,
  base_asset_volume: Volume,
  quote_asset_volume_left: Volume,
  maker_fee: Fraction,
});
export type Order = z.infer<typeof Order>;
