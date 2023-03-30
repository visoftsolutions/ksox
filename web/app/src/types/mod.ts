import { z } from "zod";
import { Uuid } from "~/types/primitives/uuid";
import { Volume } from "~/types/primitives/volume";
import { CandlestickType } from "./candlestick";
import { Pagination } from "./primitives/pagination";

export const DepthRequest = z.object({
  quoteAssetId: Uuid,
  baseAssetId: Uuid,
  precision: z.number().optional(),
  limit: z.number().optional(),
});

export type DepthRequest = z.infer<typeof DepthRequest>;

export const OhlcvRequest = z.object({
  quoteAssetId: Uuid,
  baseAssetId: Uuid,
  kind: z.nativeEnum(CandlestickType).optional(),
  referencePoint: z.date(),
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
  quoteAssetId: Uuid,
  baseAssetId: Uuid,
  pagination: z.optional(Pagination),
});

export type TradesRequest = z.infer<typeof TradesRequest>;

export const SubmitRequest = z.object({
  quoteAssetId: Uuid,
  baseAssetId: Uuid,
  quoteAssetVolume: Volume,
  baseAssetVolume: Volume,
});
export type SubmitRequest = z.infer<typeof SubmitRequest>;

export const MintBurnRequest = z.object({
  assetId: Uuid,
  amount: Volume,
});

export type MintBurnRequest = z.infer<typeof MintBurnRequest>;

export const Session = z.object({});

export const CancelRequest = z.object({
  orderId: Uuid,
});
export type CancelRequest = z.infer<typeof CancelRequest>;
