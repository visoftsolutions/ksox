import { createContext, JSX, useContext } from "solid-js";
import { createStore } from "solid-js/store";

export interface Network {
  icon: string;
  name: string;
}

export interface Token {}

interface BucketInfo {
  durationDays: string;
  durationHours: string;
  durationMinutes: string;
  durationSeconds: string;
  overrideThreshold: string;
}

export interface CrowdsaleProvider {
  phaseName: string;
  status: boolean;
  showModal: boolean;
  available_networks: Network[];
  available_tokens: Token[];
  selected_network: Network | undefined;
  selected_token: Token | undefined;
  timerDays: string | undefined;
  timerHours: string | undefined;
  timerMinutes: string | undefined;
  timerSeconds: string | undefined;
  openBucketNo: string | undefined;
  totalBucketNo: string | undefined;
  phaseSupply: number | undefined;
  bucketSupply: number | undefined;
  bucketInfo: BucketInfo | undefined;
}

export const [crowdsale, setCrowdsale] = createStore<CrowdsaleProvider>({
  phaseName: "Phase I",
  status: true,
  showModal: false,
  available_networks: [
    { icon: "/gfx/asset_icons/eth.svg", name: "ethereum" },
    { icon: "/gfx/asset_icons/matic.svg", name: "polygon" },
  ],
  available_tokens: [{}],
  selected_network: undefined,
  selected_token: undefined,
  timerDays: undefined,
  timerHours: undefined,
  timerMinutes: undefined,
  timerSeconds: undefined,
  openBucketNo: undefined,
  totalBucketNo: undefined,
  phaseSupply: 0.3371,
  bucketSupply: 0.7192,
  bucketInfo: undefined,
});
const CrowdsaleContext = createContext<CrowdsaleProvider>(crowdsale);
export function CrowdsaleProvider(props: { children: JSX.Element }) {
  return (
    <CrowdsaleContext.Provider value={crowdsale}>
      {props.children}
    </CrowdsaleContext.Provider>
  );
}
export function useCrowdsale() {
  return useContext<CrowdsaleProvider>(CrowdsaleContext);
}
