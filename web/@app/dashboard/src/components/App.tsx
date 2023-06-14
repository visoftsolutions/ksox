import { BadgeFamily } from "@web/types/badge";
import Badges from "./Badges";
import Profile from "./Profile";

export default function App() {
  return (
    <div class="m-auto grid max-w-2xl grid-rows-[auto_1fr] items-center justify-stretch gap-10 md:max-w-7xl">
      <Profile
        name={"Okm165"}
        publicWallet={"0x21B17b25b864659F2947a80B2A8E36f372f1D945"}
        badges={[{ name: "Crypto Connoisseur" }, { name: "Crypto Connoisseur" }, { name: "Crypto Connoisseur" }]}
      />
      <Badges badgeFamilies={[BadgeFamily.ValutBadge, BadgeFamily.TradeBadge, BadgeFamily.TransferBadge, BadgeFamily.MakerBadge, BadgeFamily.TakerBadge]}/>
    </div>
  );
}
