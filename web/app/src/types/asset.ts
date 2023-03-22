import { z } from "zod";
import { Fraction } from "~/types/primitives/fraction";
import { Datetime } from "./primitives/datetime";
import { Uuid } from "./primitives/uuid";

export const Asset = z.object({
  id: Uuid,
  created_at: Datetime,
  name: z.string(),
  symbol: z.string(),
  maker_fee: Fraction,
  taker_fee: Fraction,
});
export type Asset = z.infer<typeof Asset>;
