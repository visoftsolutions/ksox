import { z } from "zod";
import { Uuid } from "./primitives/uuid";
import { CandlestickType } from "./candlestick";
import { Pagination } from "./primitives/pagination";
import { Fraction } from "./primitives/fraction";

export const DepthRequest = z.object({
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  precision: z.number().optional(),
  limit: z.number().optional(),
});

export type DepthRequest = z.infer<typeof DepthRequest>;

export const OhlcvRequest = z.object({
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  kind: z.nativeEnum(CandlestickType).optional(),
  reference_point: z.date(),
  span: z.number(),
});

export type OhlcvRequest = z.infer<typeof OhlcvRequest>;

export const SearchRequest = z.object({
  input: z.string(),
});

export type SearchRequest = z.infer<typeof SearchRequest>;

export const AssetResponse = z.object({
  id: Uuid,
  name: z.string(),
  symbol: z.string(),
});

export type AssetResponse = z.infer<typeof AssetResponse>;

export const TradesRequest = z.object({
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  pagination: z.optional(Pagination),
});

export type TradesRequest = z.infer<typeof TradesRequest>;

export const SubmitRequest = z.object({
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  price: Fraction,
  quote_asset_volume: Fraction,
  presentation: z.boolean(),
});
export type SubmitRequest = z.infer<typeof SubmitRequest>;

export const MintBurnRequest = z.object({
  asset_id: Uuid,
  amount: Fraction,
});

export type MintBurnRequest = z.infer<typeof MintBurnRequest>;

export const Session = z.object({});

export const CancelRequest = z.object({
  order_id: Uuid,
});
export type CancelRequest = z.infer<typeof CancelRequest>;

export const TransferRequest = z.object({
  taker_id: Uuid,
  asset_id: Uuid,
  amount: Fraction,
});

export type TransferRequest = z.infer<typeof TransferRequest>;

export const MeRequest = z.object({
  name: z.nullable(z.string()),
  email: z.nullable(z.string()),
  phone: z.nullable(z.string()),
});

export type MeRequest = z.infer<typeof MeRequest>;
