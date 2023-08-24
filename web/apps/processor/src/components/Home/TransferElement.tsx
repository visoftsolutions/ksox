import { Asset } from "@packages/types/asset";
import { formatDate } from "@packages/utils/formatDate";
import { User } from "@packages/types/user";
import firstLastChars from "@packages/utils/firstLastChars";

export interface ITransferElement {
  user: User;
  date: Date;
  amount: number;
  asset: Asset;
}

export default function TransferElement(props: ITransferElement) {
  return (
    <div class="rounded-xl cursor-pointer">
      <div class="grid grid-cols-[1fr_auto]">
        <div class="grid grid-rows-2">
          <p class="text-r-light-text dark:text-r-dark-text font-sans font-bold">
            {props.user.name ?? firstLastChars(props.user.address, 8, 8)}
          </p>
          <p class="font-sans text-xs text-r-dark-secondary-text">
            {`${props.date.getHours().toString().padStart(2, "0")}:${props.date
              .getMinutes()
              .toString()
              .padStart(2, "0")} ${formatDate(props.date)}`}
          </p>
        </div>
        <div class="grid grid-rows-2 justify-items-end items-center">
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
