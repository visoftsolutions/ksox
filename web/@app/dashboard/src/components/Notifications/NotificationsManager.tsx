import { For, JSX, createSignal, onMount } from "solid-js";
import { INotification } from "./Notification";
import MyNotification from "./Notification";

export interface INotificationManager {
  timer?: number; // time after which the notification will disappear
}

/* Wrapper component that should wrap the App component. Used for managing notificitations i.e. pushing a new one */

export default function NotificationsManager(props: INotificationManager & { children?: JSX.Element }) {
  const { timer = 8 } = props;
  const [notifications, setNotifications] = createSignal<INotification[]>([]);

  // used for adding notifications, adding interface might change once we have a backend
  const pushNotification = (notification: INotification) => {
    notification.id = Number(Math.random() * 1000000);
    setNotifications([...notifications(), {...notification, timer: timer}]);
    setTimeout(() => {
      const index = notifications().findIndex((i) => i.id === notification.id);
      if (index !== -1) {
        notifications().splice(index, 1);
        // setNotifications(notifications().splice(index, 1));
      }
    }, timer * 1000);
  };
  // store, list, linked hashmap

  const goToSite = (url: string) => {window.location.assign(url);};

  // demo
  onMount(() => {
    pushNotification({ text: "Small notification test. Clicking on it resolves in no action, and the cursor doesn't become a pointer."});
    setTimeout(() => {pushNotification({ text: "Medium notification test. Click on the notification to console.log()", imgPath: "gfx/placeholderBadge3.png", onAction: () => {console.log('test')} });}, 2000);
    setTimeout(() => {pushNotification({ text: "Big notification test. Click on the notification to go to some test link.", imgPath: "gfx/placeholderBadge2.png", type: "big", onAction: () => {goToSite("https://github.com/Fifus17/Travel-Agency-App/blob/master/README.md")} });}, 4000);
  });

  return (
    <div class="h-screen w-screen">
      {/* Content wrapper */}
      {/* Content */}
      <div>{props.children}</div>
      {/* Overlay */}
      <div class="absolute top-0 z-10 grid h-screen w-screen grid-cols-5 grid-rows-5">
        {/* Notifications container */}
        <div class="col-span-1 row-start-5 flex flex-col-reverse">{/* Showing notifications */}
        <For each={notifications()}>
        {(item, index) => (
          <MyNotification {...item} />
        )}
      </For>
        </div>
      </div>
    </div>
  );
}
