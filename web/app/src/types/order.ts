import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Fraction } from "./primitives/fraction";
import { Uuid } from "./primitives/uuid";
import { Volume } from "./primitives/volume";

export const Order = z.object({
  id: Uuid,
  createdAt: Datetime,
  userId: Uuid,
  isActive: z.boolean(),
  quoteAssetId: Uuid,
  baseAssetId: Uuid,
  quoteAssetVolume: Volume,
  baseAssetVolume: Volume,
  quoteAssetVolumeLeft: Volume,
  makerFee: Fraction,
});
export type Order = z.infer<typeof Order>;
