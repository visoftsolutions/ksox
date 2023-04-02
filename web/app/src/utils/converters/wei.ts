import { multiplyFloat } from "../multiplyFloat";

export function toWei(number: number): bigint {
  // accurate
  return multiplyFloat(1000000000000000000n, number)
}

export function fromWei(number: bigint): number {
  // not accurate
  return Number(number) / Number(1000000000000000000n);
}
