import { z } from "zod";
import { Volume } from "./volume";
import { fromWei } from "~/utils/converters/wei";

export const Fraction = z.object({
  numerator: Volume,
  denominator: Volume,
});
export type Fraction = z.infer<typeof Fraction>;

export function evaluate(f: Fraction): number {
  return fromWei(f.numerator) / fromWei(f.denominator);
}

export function evaluateInv(f: Fraction): number {
  return fromWei(f.denominator) / fromWei(f.numerator);
}
