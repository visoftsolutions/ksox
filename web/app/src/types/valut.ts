import { z } from "zod";
import { Uuid } from "./primitives/uuid";
import { Volume } from "./primitives/volume";

export const Valut = z.object({
  id: Uuid,
  userId: Uuid,
  assetId: Uuid,
  balance: Volume,
});
export type Valut = z.infer<typeof Valut>;
