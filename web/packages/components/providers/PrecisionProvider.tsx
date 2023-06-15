import { Accessor, createContext, createSignal, JSX, useContext } from "solid-js";

export const [precision, setPrecision] = createSignal<number>(3);
const PrecisionContext = createContext<Accessor<number>>(precision);
export function PrecisionProvider(props: { children: JSX.Element }) {
  return <PrecisionContext.Provider value={precision}>{props.children}</PrecisionContext.Provider>;
}
export function usePrecision() {
  return useContext<Accessor<number>>(PrecisionContext);
}
