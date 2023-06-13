import { z } from "zod";

enum Badges {
  FirstTaker,
  TakerBeginner,
  TakerBandit,
  TakerBoss,
  TakerBaron,
  TakerBandwagon,
  TakerBehemoth,
}

// -- ORDERS BADGES
// -- badge for performing N bids:

export const TakerBadge = z.nativeEnum(Badges);
export type TakerBadge = z.infer<typeof TakerBadge>;
