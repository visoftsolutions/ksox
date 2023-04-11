import { z } from "zod";
import { Uuid } from "./primitives/uuid";
import { Volume } from "./primitives/volume";
import { Datetime } from "./primitives/datetime";

export const Valut = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  user_id: Uuid,
  asset_id: Uuid,
  balance: Volume,
});
export type Valut = z.infer<typeof Valut>;
