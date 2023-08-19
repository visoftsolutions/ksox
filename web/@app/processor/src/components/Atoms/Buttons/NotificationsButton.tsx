import { createSignal } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import BellIcon from "../Icons/BellIcon";
import Icon from "../Icons/Icon";
import { Palette } from "../Palette";
import { useDarkModeContext } from "~/components/providers/DarkModeProvider";

export interface INotificationsButton {
  status?: boolean;
  className?: string;
}

export default function NotificationsButton(props: INotificationsButton) {
  const darkMode = useDarkModeContext();
  const [status, setStatus] = createSignal(props.status);

  const toggleStatus = () => {
    setStatus(!status());
  };

  return (
    <button
      class={`relative h-8 w-8 p-0 ${props.className}`}
      onClick={toggleStatus}
    >
      <div class={darkMode.darkMode() ? "" : "hidden"}>
        <Icon
          icon={
            BellIcon({ stroke: Palette["r-dark-text"] })
          }
        />
      </div>
      <div class={darkMode.darkMode() ? "hidden" : ""}>
        <Icon
          icon={
            BellIcon({ stroke: Palette["r-light-text"] })
          }
        />
      </div>
      {status() ? (
        <div class="z-100 absolute right-1 top-0 h-1 w-1 rounded-full bg-red"></div>
      ) : null}
    </button>
  );
}
