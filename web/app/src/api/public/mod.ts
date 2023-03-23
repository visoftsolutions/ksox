import { z } from "zod";
import { Uuid } from "~/types/primitives/uuid";
import { Volume } from "~/types/primitives/volume";
import { BASE_URL, Pagination } from "../mod";

export const PUBLIC_URL = BASE_URL + "/public";

export const DepthRequest = z.object({
  quote_asset_id: Uuid,
  base_asset_id: Uuid,
  precision: z.number().optional(),
  limit: z.number().optional(),
});

export type DepthRequest = z.infer<typeof DepthRequest>;

export const PriceLevel = z.object({
  price: z.number(),
  volume: Volume,
});

export type PriceLevel = z.infer<typeof PriceLevel>;

export enum CandlestickType {
  Interval,
  Tick,
}

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
