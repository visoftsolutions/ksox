import { Asset } from "@packages/types/asset";
import { formatDate } from "@packages/utils/formatDate";
import Picture from "~/components/Atoms/Picture";
import { User } from "@packages/types/user";
import firstLastChars from "@packages/utils/firstLastChars";

export interface ITransfer {
  user: User;
  date: Date;
  amount: number;
  asset: Asset;
}

export default function Transfer(props: ITransfer) {
  return (
    <div class="rounded-xl bg-r-light-foreground active:bg-r-light-background dark:bg-r-dark-foreground dark:active:bg-r-dark-active cursor-pointer">
      <div class="flex justify-between">
        <div class="m-4 flex">
          <div class="ml-4">
            <p class="text-r-light-text dark:text-r-dark-text font-sans font-bold">
              {props.user.name ?? firstLastChars(props.user.address, 8, 8)}
            </p>
            <p class="font-sans text-xs text-r-dark-secondary-text">
              {`${formatDate(props.date)}, ${props.date
                .getHours()
                .toString()
                .padStart(2, "0")}:${props.date
                .getMinutes()
                .toString()
                .padStart(2, "0")}`}
            </p>
          </div>
        </div>
        <div class="m-4 flex flex-col items-end">
          <p class="text-r-light-text dark:text-r-dark-text font-sans ">
            {`${props.amount > 0 ? "+" : ""}${props.amount}`}
          </p>
          <p class="font-sans text-xs text-r-dark-secondary-text">
            {props.asset.symbol}
          </p>
        </div>
      </div>
    </div>
  );
}
