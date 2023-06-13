import { z } from "zod";

enum Badges {
  FirstToken,
  TokenTourist,
  TokenCollector,
  TokenTamer,
  TokenHoarder,
  TokenDiversifier,
  TokenMagnate,
  TokenOverlord,
  TokenTitan,
}

// -- VALUTS BADGES
// -- badge for having N diffirent non zero valuts:

export const ValutBadge = z.nativeEnum(Badges);
export type ValutBadge = z.infer<typeof ValutBadge>;
