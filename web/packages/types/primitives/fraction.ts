import { z } from "zod";

export const Fraction = z.object({
  numer: z.coerce.bigint(),
  denom: z.coerce.bigint(),
});
export type Fraction = z.infer<typeof Fraction>;

export function finv(f: Fraction) {
  return Fraction.parse({ numer: f.denom, denom: f.numer });
}

export function fmul(a: Fraction, b: Fraction) {
  return Fraction.parse({ numer: a.numer * b.numer, denom: a.denom * b.denom });
}

export function fFromBigint(n: bigint) {
  return Fraction.parse({ numer: n, denom: 1n });
}

export function fmax(a: Fraction, b: Fraction) {
  return a.numer * b.denom > a.denom * b.numer ? a : b;
}

export function fmin(a: Fraction, b: Fraction) {
  return a.numer * b.denom < a.denom * b.numer ? a : b;
}

export function ev(f: Fraction) {
  return Number(f.numer) / Number(f.denom);
}
