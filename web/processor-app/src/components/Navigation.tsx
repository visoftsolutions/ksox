import { A } from "solid-start";
import NavButton from "./Navigation/NavButton";
import { useNav, Nav } from "~/utils/providers/NavProvider";

export default function Navigation() {
  const nav = useNav();
  return (
    <div class="grid grid-cols-2 bg-gray-2">
      <A href="/">
        <NavButton name="Home" imgPath="/gfx/home.svg" highlighted={nav() == Nav.App} />
      </A>
      <A href="/transfer">
        <NavButton name="Transfer" imgPath="/gfx/transfer.svg" highlighted={nav() == Nav.Transfer} />
      </A>
    </div>
  );
}
