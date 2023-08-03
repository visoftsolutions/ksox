import { z } from "zod";
import { Fraction } from "./fraction";

enum ValueType {
  Finite = "Finite",
  Infinite = "Infinite",
}

export const Value = z.record(z.nativeEnum(ValueType), Fraction);
export type Value = z.infer<typeof Value>;
