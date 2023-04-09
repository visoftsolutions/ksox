import { z } from "zod";
import { Volume } from "./volume";

export const Fraction = z.object({
  numerator: Volume,
  denominator: Volume,
});
export type Fraction = z.infer<typeof Fraction>;

export function finv(f: Fraction): Fraction {
  return { numerator: f.denominator, denominator: f.numerator };
}

export function fmul(a: Fraction, b: Fraction): Fraction {
  return { numerator: a.numerator * b.numerator, denominator: a.denominator * b.denominator };
}

export function fFromBigint(n: bigint): Fraction {
  return { numerator: n, denominator: 1n };
}

export function fmax(a: Fraction, b: Fraction): Fraction {
  return a.numerator * b.denominator > a.denominator * b.numerator ? a : b;
}

export function fmin(a: Fraction, b: Fraction): Fraction {
  return a.numerator * b.denominator < a.denominator * b.numerator ? a : b;
}

export function ev(f: Fraction): number {
  return Number(f.numerator) / Number(f.denominator);
}
