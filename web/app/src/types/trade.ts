import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Fraction } from "./primitives/fraction";
import { Direction } from "./primitives/direction";

export const PublicTrade = z.object({
  price: Fraction,
  volume: Fraction,
  time: Datetime,
  direction: Direction,
});
export type PublicTrade = z.infer<typeof PublicTrade>;
