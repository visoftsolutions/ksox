import { z } from "zod";

enum Badges {
  FirstTrade,
  TradeNovice,
  TradeTornado,
  TradeTyrant,
  TradeMogul,
  TradeMagnate,
  TradeOverlord,
  TradeLegend,
  TradeTitan,
}

// -- TRADES BADGES
// -- badge for performing N trades:

export const TradeBadge = z.nativeEnum(Badges);
export type TradeBadge = z.infer<typeof TradeBadge>;
