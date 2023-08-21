import { createSignal } from "solid-js";
import Currency, { ICurrency } from "./Currency";
import { useCurrencyContext } from "~/components/providers/CurrencyProvider";

export interface ICurrencies {
  currencies: ICurrency[];
}

export default function Currencies(props: ICurrencies) {
  const currentCurrency = useCurrencyContext();

  return (
    <div class="relative rounded-xl bg-r-light-foreground dark:bg-r-dark-modal-foreground p-1 max-h-[40vh] overflow-scroll overflow-y-auto scrollbar-thumb-r-dark-secondary-text dark:scrollbar-thumb-r-dark-active">
      {props.currencies.length > 0 ? (
        props.currencies.map((currency, index) => (
          <Currency
            name={currency.name}
            img={currency.img}
            amount={currency.amount}
            symbol={currency.symbol}
            selected={currentCurrency.currency().symbol == currency.symbol}
            onClick={() => {
              currentCurrency.setCurrency(currency);
            }}
          />
        ))
      ) : (
        <p class="text-r-light-text dark:text-r-dark-text">
          You have no currencies.
        </p>
      )}
    </div>
  );
}
