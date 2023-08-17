import {
  Accessor,
  Setter,
  createContext,
  createSignal,
  JSX,
  useContext,
} from "solid-js";
import { ICurrency } from "../Atoms/Currencies/Currency";

interface CurrencyContextValue {
  currency: Accessor<ICurrency>;
  setCurrency: Setter<ICurrency>;
}

const CurrencyContext = createContext<CurrencyContextValue>();

export function CurrencyProvider(props: any) {
  const [currencyValue, setCurrencyValue] = createSignal<ICurrency>({
    name: "Bitcoin",
    symbol: "BTC",
    amount: 0,
    img: "gfx/bitcoin_placeholder.png",
    selected: false,
  });

  const contextValue: CurrencyContextValue = {
    currency: currencyValue,
    setCurrency: setCurrencyValue,
  };

  return (
    <CurrencyContext.Provider value={contextValue}>
      {props.children}
    </CurrencyContext.Provider>
  );
}

export const useCurrencyContext = () => useContext(CurrencyContext)!;
