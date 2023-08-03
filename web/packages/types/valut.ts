import { z } from "zod";
import { Uuid } from "./primitives/uuid";
import { Datetime } from "./primitives/datetime";
import { Value } from "./primitives/value";

export const Valut = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  user_id: Uuid,
  asset_id: Uuid,
  balance: Value,
});
export type Valut = z.infer<typeof Valut>;
