import { z } from "zod";
import { Fraction } from "./primitives/fraction";

export const PriceLevel = z.object({
  price: Fraction,
  volume: Fraction,
});

export type PriceLevel = z.infer<typeof PriceLevel>;
