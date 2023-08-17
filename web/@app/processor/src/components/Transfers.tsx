import Transfer, { ITransfer } from "./Transfer";

export interface ITransfers {
  transfers: ITransfer[];
}

export default function Transfers(props: ITransfers) {
  return (
    <div class="rounded-xl bg-r-light-foreground dark:bg-r-dark-foreground m-6 my-2">
      {props.transfers.map((transfer) => (
        <Transfer
          name={transfer.name}
          img={transfer.img}
          date={transfer.date}
          text={transfer.text}
        />
      ))}
    </div>
  );
}
