import {
  Accessor,
  createContext,
  createSignal,
  JSX,
  useContext,
} from "solid-js";

export interface CrowdsaleProvider {
  phaseName: string;
  status: boolean;
}

export const [crowdsale, setNav] = createSignal<CrowdsaleProvider>({
  phaseName: "Phase1",
  status: true,
});
const CrowdsaleContext = createContext<Accessor<CrowdsaleProvider>>(crowdsale);
export function CrowdsaleProvider(props: { children: JSX.Element }) {
  return (
    <CrowdsaleContext.Provider value={crowdsale}>
      {props.children}
    </CrowdsaleContext.Provider>
  );
}
export function useCrowdsale() {
  return useContext<Accessor<CrowdsaleProvider>>(CrowdsaleContext);
}
