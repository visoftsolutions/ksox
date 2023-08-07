import { A } from "solid-start";
import NavButton from "./Navigation/NavButton";
import { useNav, Nav } from "~/components/providers/NavProvider";

export default function Navigation() {
  const nav = useNav();
  return (
    <div class="grid grid-cols-2 bg-gray-2">
      <A href="/">
        <NavButton
          name="Home"
          imgPath="/gfx/home.svg"
          highlighted={nav() == Nav.App}
        />
      </A>
      <A href="/account">
        <NavButton
          name="Account"
          imgPath="/gfx/user.svg"
          highlighted={nav() == Nav.Account}
        />
      </A>
    </div>
  );
}
