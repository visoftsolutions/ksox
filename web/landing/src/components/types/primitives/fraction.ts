export type Fraction = {
  numer: bigint;
  denom: bigint;
};

export function finv(f: Fraction): Fraction {
  return { numer: f.denom, denom: f.numer };
}

export function fmul(a: Fraction, b: Fraction): Fraction {
  return { numer: a.numer * b.numer, denom: a.denom * b.denom };
}

export function fFromBigint(n: bigint): Fraction {
  return { numer: n, denom: 1n };
}

export function fmax(a: Fraction, b: Fraction): Fraction {
  return a.numer * b.denom > a.denom * b.numer ? a : b;
}

export function fmin(a: Fraction, b: Fraction): Fraction {
  return a.numer * b.denom < a.denom * b.numer ? a : b;
}

export function ev(f: Fraction): number {
  return Number(f.numer) / Number(f.denom);
}
