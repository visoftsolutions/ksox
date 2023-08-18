import { A } from "solid-start";
import NavButton from "../Navigation/NavButton";
import { useNav, Nav } from "~/components/providers/NavProvider";
import TransferIcon from "../Atoms/Icons/TransferIcon";
import HomeIcon from "../Atoms/Icons/HomeIcon";
import SettingsIcon from "../Atoms/Icons/SettingsICon";
import SideNavButton from "../Navigation/SideNavButton";
import BellIcon from "../Atoms/Icons/BellIcon";

export default function SideNavigation() {
  const nav = useNav();
  return (
    <div class="grid grid-rows-3 ">
      <A href="/">
        <SideNavButton
          name="Home"
          highlighted={nav() == Nav.Home}
          icon={HomeIcon({})}
        />
      </A>
      <A href="/1">
        <SideNavButton
          name="Transfer"
          highlighted={nav() == Nav.Transfer}
          icon={TransferIcon({})}
        />
      </A>
      <A href="/2">
        <SideNavButton
          name="Notifications"
          highlighted={nav() == Nav.Notifications}
          icon={BellIcon({})}
        />
      </A>
      <A href="/3">
        <SideNavButton
          name="Settings"
          highlighted={nav() == Nav.Settings}
          icon={SettingsIcon({})}
        />
      </A>
    </div>
  );
}