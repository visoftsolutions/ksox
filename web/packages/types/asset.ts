import { z } from "zod";
import { Fraction } from "./primitives/fraction";
import { Datetime } from "./primitives/datetime";
import { Uuid } from "./primitives/uuid";
import { Address } from "@web/components/providers/SessionProvider/models";

export const Asset = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  name: z.string(),
  symbol: z.string(),
  address: Address,
  maker_fee: Fraction,
  taker_fee: Fraction,
});
export type Asset = z.infer<typeof Asset>;
