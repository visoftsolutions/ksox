import { z } from "zod";
import { Volume } from "./volume";

export const PriceLevel = z.object({
  price: z.number(),
  volume: Volume,
});

export type PriceLevel = z.infer<typeof PriceLevel>;
