import { Index } from "solid-js";
import TransferElement, {
  ITransferElement,
} from "~/components/Transfer/TransferElement";

export interface ITransfers {
  transfers: ITransferElement[];
}

export default function Transfers(props: ITransfers) {
  return (
    <div class="rounded-xl bg-r-light-foreground dark:bg-r-dark-foreground scrollbar-thumb-r-dark-secondary-text dark:scrollbar-thumb-r-dark-active">
      <Index each={props.transfers}>
        {(element) => (
          <TransferElement
            name={element().name}
            img={element().img}
            date={element().date}
            text={element().text}
          />
        )}
      </Index>
    </div>
  );
}
