import { z } from "zod";
import { Volume } from "./volume";

export const Fraction = z.object({
  numerator: Volume,
  denominator: Volume,
});
export type Fraction = z.infer<typeof Fraction>;
