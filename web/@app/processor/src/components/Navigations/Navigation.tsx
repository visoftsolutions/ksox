import { A } from "solid-start";
import NavButton from "../Navigation/NavButton";
import { useNav, Nav } from "~/components/providers/NavProvider";
import TransferIcon from "../Atoms/Icons/TransferIcon";
import HomeIcon from "../Atoms/Icons/HomeIcon";

export default function Navigation() {
  const nav = useNav();
  return (
    <div class="grid grid-cols-2 ">
      <A href="/">
        <NavButton
          name="Home"
          highlighted={nav() == Nav.Home}
          icon={HomeIcon({})}
        />
      </A>
      <A href="/1">
        <NavButton
          name="Transfer"
          highlighted={nav() == Nav.Transfer}
          icon={TransferIcon({})}
        />
      </A>
    </div>
  );
}
