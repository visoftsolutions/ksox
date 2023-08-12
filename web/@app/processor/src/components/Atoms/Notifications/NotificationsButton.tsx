import { createSignal } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export interface INotificationsButton {
    status?: boolean;
    className?: string;
}

export default function NotificationsButton(props: INotificationsButton) {
    const [status, setStatus] = createSignal(props.status);

    const toggleStatus = () => {
        setStatus(!status());
    }

    return (
            <button class={`relative w-8 h-8 p-0 ${props.className}`} /*onClick={toggleStatus}*/>
                <img src={joinPaths(base, "gfx/bell.svg")} alt="Notifications Icon" class="w-8 h-8 p-0"/>
                {status() ? <div class="absolute top-0 right-1 w-1 h-1 bg-red rounded-full z-100"></div> : null}
            </button>
    )
}