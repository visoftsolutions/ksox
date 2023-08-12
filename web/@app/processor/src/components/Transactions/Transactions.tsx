import Transaction, { ITransaction } from "./Transaction";

export interface ITransactions {
  transactions: ITransaction[];
  noTransactions?: number;
}

export default function Transactions(props: ITransactions) {

  return (
    <div class="rounded-xl bg-r-dark-foreground">
      {props.transactions.length > 0 ? (
        props.transactions.map((transaction, index) => (
          <Transaction
            title={transaction.title}
            img={transaction.img}
            date={transaction.date}
            hour={transaction.hour}
            amount={transaction.amount}
            plus={transaction.plus}
            currency={transaction.currency}
          />
        ))
      ) : (
        <div class="p-4">
          {props.noTransactions ? (
            Array.from({ length: props.noTransactions }).map((_, index) => (
              <Transaction
                title="No Transaction"
                img="placeholder.png"
                date=""
                hour=""
                amount={0}
                plus={false}
                currency=""
              />
            ))
          ) : (
            <p>No transactions available.</p>
          )}
        </div>
      )}
    </div>
  );
}
