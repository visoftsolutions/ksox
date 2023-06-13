import { z } from "zod";

enum Badges {
  FirstTransfer,
  TransferRookie,
  TransferTrooper,
  TransferCourier,
  TransferMagician,
  TransferWizard,
  TransferTerminator,
  TransferLegend,
  TransferTitan,
}

// -- TRADES BADGES
// -- badge for performing N transfers:

export const TransferBadge = z.nativeEnum(Badges);
export type TransferBadge = z.infer<typeof TransferBadge>;
