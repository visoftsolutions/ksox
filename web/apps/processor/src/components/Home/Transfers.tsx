import { Index } from "solid-js";
import Transaction, { ITransfer } from "~/components/Home/Transfer";
import Transfer from "./Transfer";

export interface ITransfers {
  transfers: ITransfer[];
}

export default function Transactions(props: ITransfers) {
  return (
    <div class="rounded-xl bg-r-light-foreground dark:bg-r-dark-foreground">
      <Index each={props.transfers}>
        {(element) => (
          <Transfer
            user={element().user}
            date={element().date}
            amount={element().amount}
            asset={element().asset}
          />
        )}
      </Index>
    </div>
  );
}
