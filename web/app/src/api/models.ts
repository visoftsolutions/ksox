import { z } from "zod";

export const BASE_URL = "/api"

export const Pagination = z.object({
    limit: z.number().nonnegative(),
    offset: z.number().nonnegative(),
})
export type Pagination = z.infer<typeof Pagination>;

export const MintBurnRequest = z.object({
    asset_id: z.string().uuid(),
    amount: z.number(),
})

export type MintBurnRequest = z.infer<typeof MintBurnRequest>;