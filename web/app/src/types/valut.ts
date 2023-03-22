import { z } from "zod";
import { Uuid } from "./primitives/uuid";
import { Volume } from "./primitives/volume";

export const Valut = z.object({
  id: Uuid,
  user_id: Uuid,
  asset_id: Uuid,
  balance: Volume,
});
export type Valut = z.infer<typeof Valut>;