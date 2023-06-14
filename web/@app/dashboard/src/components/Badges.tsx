import { Index } from "solid-js";
import { useSession } from "@web/components/providers/SessionProvider";
import CreateBadge from "./Badges/Badge";
import { Dynamic } from "solid-js/web";
import { BadgeFamily } from "@web/types/badge";

export default function Badges(props: {badgeFamilies: BadgeFamily[]}) {
  const session = useSession();

  return (
    <div class="grid grid-cols-[repeat(auto-fit,minmax(300px,1fr))] gap-8 md:grid-cols-[repeat(auto-fit,minmax(500px,1fr))]">
      <Index each={props.badgeFamilies}>
        {(element) => <Dynamic component={CreateBadge(session(), element())} />}
      </Index>
    </div>
  );
}
