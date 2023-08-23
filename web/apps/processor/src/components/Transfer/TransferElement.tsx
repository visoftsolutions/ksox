import Picture from "~/components/Atoms/Picture";
import { formatDate } from "@packages/utils/formatDate";

export interface ITransferElement {
  name: string;
  img: string;
  date: Date; // i.e. Aug 21, Apr 11, 2021
  text: string;
}

export default function TransferElement(props: ITransferElement) {
  return (
    <div class="rounded-xl bg-r-light-foreground dark:bg-r-dark-foreground active:bg-r-light-background dark:active:bg-r-dark-active">
      <div class="flex justify-between">
        <div class="m-4 flex">
          <Picture src={props.img} alt="test" size={42} />
          <div class="ml-4">
            <p class="text-dark-text font-sans font-bold ">{props.name}</p>
            <p class="font-sans text-xs text-r-dark-secondary-text">
              {props.text}
            </p>
          </div>
        </div>
        <div class="m-4 flex flex-col items-end">
          <p class="font-sans text-xs text-r-dark-secondary-text">
            {formatDate(props.date)}
          </p>
        </div>
      </div>
    </div>
  );
}
