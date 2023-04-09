import { Fraction } from "~/types/primitives/fraction";

export function fToWeiFloor(f: Fraction): bigint {
  const x = f.numerator * 1000000000000000000n;
  const y = f.denominator;
  return x / y;
}

export function fToWeiCeil(f: Fraction): bigint {
  const x = f.numerator * 1000000000000000000n;
  const y = f.denominator;
  return (x + y - 1n) / y;
}

export function fFromWei(n: bigint): Fraction {
  const x = n;
  const y = 1000000000000000000n;
  return { numerator: x, denominator: y };
}
