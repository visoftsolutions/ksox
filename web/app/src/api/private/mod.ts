import { z } from "zod";
import { Uuid } from "~/types/primitives/uuid";
import { Volume } from "~/types/primitives/volume";
import { BASE_URL } from "../mod";

export const PRIVATE_URL = BASE_URL + "/private";
export const COOKIE_NAME = "session_id";

export const SubmitRequest = z.object({
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  quote_asset_volume: Volume,
  base_asset_volume: Volume,
});
export type SubmitRequest = z.infer<typeof SubmitRequest>;

export const MintBurnRequest = z.object({
  asset_id: Uuid,
  amount: Volume,
});

export type MintBurnRequest = z.infer<typeof MintBurnRequest>;

export const Session = z.object({});

export const CancelRequest = z.object({
  order_id: Uuid,
});
export type CancelRequest = z.infer<typeof CancelRequest>;
