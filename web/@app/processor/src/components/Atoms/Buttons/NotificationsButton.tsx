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
    }

    return (
            <button class={`relative w-8 h-8 p-0 ${props.className}`} onClick={toggleStatus}>
                <Icon icon={darkMode.darkMode() ? BellIcon({stroke: Palette["r-dark-text"]}) : BellIcon({stroke:Palette["r-light-text"]})}/>
                {status() ? <div class="absolute top-0 right-1 w-1 h-1 bg-red rounded-full z-100"></div> : null}
            </button>
    )
}