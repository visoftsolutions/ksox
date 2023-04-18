import { z } from "zod";

export const PriceLevel = z.object({
  price: z.number(),
  volume: z.number(),
});

export type PriceLevel = z.infer<typeof PriceLevel>;
