import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Uuid } from "./primitives/uuid";
import { Volume } from "./primitives/volume";

export const Trade = z.object({
  id: Uuid,
  createdAt: Datetime,
  quoteAssetId: Uuid,
  baseAssetId: Uuid,
  takerId: Uuid,
  orderId: Uuid,
  takerQuoteVolume: Volume,
  takerBaseVolume: Volume,
  makerQuoteVolume: Volume,
  makerBaseVolume: Volume,
});
export type Trade = z.infer<typeof Trade>;
