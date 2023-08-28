import { Fraction } from "@packages/types/primitives/fraction";
import {
  Accessor,
  createContext,
  createSignal,
  JSX,
  useContext,
} from "solid-js";

export const [precision, setPrecision] = createSignal<Fraction>({
  numer: 1n,
  denom: 1000n,
});
const PrecisionContext = createContext<Accessor<Fraction>>(precision);
export function PrecisionProvider(props: { children: JSX.Element }) {
  return (
    <PrecisionContext.Provider value={precision}>
      {props.children}
    </PrecisionContext.Provider>
  );
}
export function usePrecision() {
  return useContext<Accessor<Fraction>>(PrecisionContext);
}
