import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Fraction } from "./primitives/fraction";
import { Uuid } from "./primitives/uuid";
import { Direction } from "./primitives/direction";

export const PrivateOrder = z.object({
  id: Uuid,
  created_at: Datetime,
  is_active: z.boolean(),
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  price: Fraction,
  quote_asset_volume: Fraction,
  quote_asset_volume_left: Fraction,
  maker_fee: Fraction,
  direction: Direction,
});
export type PrivateOrder = z.infer<typeof PrivateOrder>;
