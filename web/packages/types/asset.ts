import { z } from "zod";
import { Fraction } from "./primitives/fraction";
import { Datetime } from "./primitives/datetime";
import { Uuid } from "./primitives/uuid";
import { Address } from "@packages/components/providers/SessionProvider/models";

export const Asset = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  name: z.string(),
  icon_path: z.string(),
  symbol: z.string(),
  address: Address,
  decimals: Fraction,
  maker_fee: Fraction,
  taker_fee: Fraction,
  transfer_fee: Fraction,
});
export type Asset = z.infer<typeof Asset>;
