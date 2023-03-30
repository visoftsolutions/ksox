import { z } from "zod";
import { Fraction } from "~/types/primitives/fraction";
import { Datetime } from "./primitives/datetime";
import { Uuid } from "./primitives/uuid";

export const Asset = z.object({
  id: Uuid,
  createdAt: Datetime,
  name: z.string(),
  symbol: z.string(),
  makerFee: Fraction,
  takerFee: Fraction,
});
export type Asset = z.infer<typeof Asset>;
