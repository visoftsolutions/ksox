import { JSX, createEffect, createSignal } from "solid-js";
import SmallNotification from "./SmallNotification";
import MediumNotification from "./MediumNotification";
import BigNotification from "./BigNotification";
import CustomNotification from "./CustomNotification";

export interface INotification {
  id?: number;
  timer: number;
  text: string;
  imgPath?: string;
  onAction?: () => void;
  type?: "small" | "medium" | "big" | "custom";
  custom?: JSX.Element;
}


export default function Notification(props: INotification) {

  const render = () => {
    if (props.type) {
      switch (props.type) {
        case "small":
          return <SmallNotification {...props} />;
        case "medium":
          return <MediumNotification {...props} />;
        case "big":
          return <BigNotification {...props} />;
        case "custom":
          return <CustomNotification {...props} />;
      }
    } else {
      if (props.custom) return <CustomNotification {...props} />;
      if (props.imgPath) return <MediumNotification {...props} />;
      return <SmallNotification {...props} />;
    }
  };
  return render();
}
