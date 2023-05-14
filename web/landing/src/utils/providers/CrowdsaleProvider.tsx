import { createContext, JSX, useContext } from "solid-js";
import { createStore } from "solid-js/store";

export interface PhaseState {
  isPhaseActive: boolean;
  bucketStartTimestamp: bigint;
  bucketFinishTimestamp: bigint;
  currentBucketId: bigint;
  currentBucketAmountToSell: bigint;
  prevAmountSold: bigint;
  amountSold: bigint;
  isBucketActive: boolean;
  rate: number | undefined;
}

export interface Timer {
  direction: boolean;
  timerDays: number;
  timerHours: number;
  timerMinutes: number;
  timerSeconds: number;
}

export interface CrowdsaleProvider {
  phaseState: PhaseState;
  phaseName: string;
  showModal: boolean;
  timer: Timer | undefined;
  tokenAmount: number | undefined;
}

export const [crowdsale, setCrowdsale] = createStore<CrowdsaleProvider>({
  phaseName: "Phase I",
  showModal: false,
  timer: undefined,
  tokenAmount: undefined,
  phaseState: {
    isPhaseActive: false,
    bucketStartTimestamp: 0n,
    bucketFinishTimestamp: 0n,
    currentBucketId: 0n,
    currentBucketAmountToSell: 0n,
    prevAmountSold: 0n,
    amountSold: 0n,
    isBucketActive: false,
    rate: undefined,
  },
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
