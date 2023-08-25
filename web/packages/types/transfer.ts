import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Fraction } from "./primitives/fraction";
import { Uuid } from "./primitives/uuid";
import { Address } from "@packages/components/providers/SessionProvider/models";

export const Transfer = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  from_valut_id: Uuid,
  from_user_id: Uuid,
  to_valut_id: Uuid,
  to_user_id: Uuid,
  asset_id: Uuid,
  amount: Fraction,
  fee: Fraction,
});
export type Transfer = z.infer<typeof Transfer>;

export const DisplayTransfer = z.object({
  id: Uuid,
  created_at: Datetime,
  from_user_id: Uuid,
  from_user_address: Address,
  from_user_name: z.string().nullable(),
  to_user_id: Uuid,
  to_user_address: Address,
  to_user_name: z.string().nullable(),
  asset_id: Uuid,
  asset_icon_path: z.string(),
  asset_name: z.string(),
  asset_symbol: z.string(),
  amount: Fraction,
  fee: Fraction,
});
export type DisplayTransfer = z.infer<typeof DisplayTransfer>;
