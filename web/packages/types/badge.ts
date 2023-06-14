import { z } from "zod";
import { Fraction } from "./primitives/fraction";

export const BadgeValue = z.object({
  name: z.string(),
  description: z.string(),
  progress: Fraction,
});

export type BadgeValue = z.infer<typeof BadgeValue>;

export enum BadgeFamily {
  ValutBadge = "ValutBadge",
  TradeBadge = "TradeBadge",
  TransferBadge = "TransferBadge",
  MakerBadge = "MakerBadge",
  TakerBadge = "TakerBadge",
}