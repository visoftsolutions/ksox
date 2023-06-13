import { z } from "zod";

enum Badges {
  FirstMaker,
  MakerApprentice,
  MakerAficionado,
  MakerAvenger,
  MakerAce,
  MakerAvalanche,
  MakerOverlord,
}

// -- ORDERS BADGES
// -- badge for performing N asks:

export const MakerBadge = z.nativeEnum(Badges);
export type MakerBadge = z.infer<typeof MakerBadge>;
