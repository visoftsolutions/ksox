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

export enum DisplayTransferDirection {
  Income = "income",
  Outcome = "outcome",
}
export const DisplayTransfer = z.object({
  id: Uuid,
  created_at: Datetime,
  user_id: Uuid,
  user_address: Address,
  user_name: z.string().nullable(),
  other_user_id: Uuid,
  other_user_address: Address,
  other_user_name: z.string().nullable(),
  asset_id: Uuid,
  asset_icon_path: z.string(),
  asset_name: z.string(),
  asset_symbol: z.string(),
  amount: Fraction,
  fee: Fraction,
  direction: z.nativeEnum(DisplayTransferDirection),
});
export type DisplayTransfer = z.infer<typeof DisplayTransfer>;
