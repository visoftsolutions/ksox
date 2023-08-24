import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Fraction } from "./primitives/fraction";
import { Uuid } from "./primitives/uuid";

export const Transfer = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  from_valut_id: Uuid,
  from_user_id: Uuid,
  to_valut_id: Uuid,
  to_user_id: Uuid,
  asset_id: Uuid,
  amount: Fraction,
  fee: Fraction,
});
export type Transfer = z.infer<typeof Transfer>;
