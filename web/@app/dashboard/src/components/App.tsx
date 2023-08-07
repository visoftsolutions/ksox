import { BadgeFamily } from "@web/types/badge";
import CreateProfile from "./App/Profile";
import { Dynamic, Index } from "solid-js/web";
import { useSession } from "@web/components/providers/SessionProvider";
import CreateBadge from "./App/Badge";

export default function App() {
  const session = useSession();

  return (
    <div class="m-auto grid max-w-2xl grid-rows-[auto_1fr] items-center justify-stretch gap-10 md:max-w-7xl">
      <Dynamic component={CreateProfile(session())} />
      <div class="grid grid-cols-[repeat(auto-fit,minmax(300px,1fr))] gap-8 md:grid-cols-[repeat(auto-fit,minmax(500px,1fr))]">
        <Index
          each={[
            BadgeFamily.ValutBadge,
            BadgeFamily.TransferBadge,
            BadgeFamily.TradeBadge,
            BadgeFamily.MakerBadge,
            BadgeFamily.TakerBadge,
          ]}
        >
          {(element) => (
            <Dynamic component={CreateBadge(session(), element())} />
          )}
        </Index>
      </div>
    </div>
  );
}
