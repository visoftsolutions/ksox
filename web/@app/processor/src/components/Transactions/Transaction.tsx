import Picture from "../Atoms/Picture";

export interface ITransaction {
  title: string;
  img: string;
  date: string; // i.e 3 July
  hour: string; // i.e 14:29
  amount: number;
  plus: boolean; // amount and plus can be combined into      amount: string
  currency: string;
  onClick?: () => void;
}

export default function Transaction(props: ITransaction) {
  return (
    <div
      class="rounded-xl bg-r-dark-foreground active:bg-r-dark-active"
    //   onClick={props.onClick || (() => {})}
    >
      <div class="flex justify-between">
        <div class="m-4 flex">
          <Picture src={props.img} alt="test" size={42} />
          <div class="ml-4">
            <p class="text-dark-text font-sans font-bold">{props.title}</p>
            <p class="font-sans text-xs text-r-dark-secondary-text">
              {props.date + ", " + props.hour}
            </p>
          </div>
        </div>
        <div class="m-4 flex flex-col items-end">
          <p class="text-dark-text font-sans ">
            {(props.plus ? "+" : "-") + props.amount}
          </p>
          <p class="font-sans text-xs text-r-dark-secondary-text">
            {props.currency}
          </p>
        </div>
      </div>
    </div>
  );
}
