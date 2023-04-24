import { z } from "zod";

export const Direction = z.enum(["buy", "sell"]);
export type Direction = z.infer<typeof Direction>;
