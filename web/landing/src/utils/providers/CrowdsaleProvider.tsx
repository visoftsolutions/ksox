import { createContext, JSX, onCleanup, onMount, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { useWallet } from "./WalletProvider";

export interface PhaseContract {
  name: string;
  isPhaseActive: boolean;
  isBucketActive: boolean;
  // bucket info
  currentBucketId: bigint;
  currentBucketStartTimestamp: bigint;
  currentBucketEndTimestamp: bigint;
  currentBucketCapacity: bigint;
  currentBucketRate: number;
  currentBucketSoldAmount: bigint;
}

export interface Timer {
  direction: boolean;
  timerDays: number;
  timerHours: number;
  timerMinutes: number;
  timerSeconds: number;
}

export interface CrowdsaleProvider {
  phaseContract: PhaseContract;
  showModal: boolean;
  timer: Timer;
  tokenAmount: number;
}

export const [crowdsale, setCrowdsale] = createStore<CrowdsaleProvider>({
  phaseContract: {
    name: "",
    isPhaseActive: false,
    isBucketActive: false,
    // bucket info
    currentBucketId: 0n,
    currentBucketStartTimestamp: 0n,
    currentBucketEndTimestamp: 0n,
    currentBucketCapacity: 0n,
    currentBucketRate: 0,
    currentBucketSoldAmount: 0n,
  },
  showModal: false,
  timer: {
    direction: false,
    timerDays: 0,
    timerHours: 0,
    timerMinutes: 0,
    timerSeconds: 0,
  },
  tokenAmount: 0,
});

type unwatch = () => void;

const CrowdsaleContext = createContext<CrowdsaleProvider>(crowdsale);
export function CrowdsaleProvider(props: { children: JSX.Element }) {
  const wallet = useWallet();

  let unwatchers: unwatch[] = [];

  onMount(async () => {
    const [
      name,
      isPhaseActive,
      isBucketActive,
      currentBucketId,
      currentBucketStartTimestamp,
      currentBucketEndTimestamp,
      currentBucketCapacity,
      currentBucketRateNumer,
      currentBucketRateDenom,
      currentBucketSoldAmount,
    ] = await Promise.all([
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "name",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "isPhaseActive",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "isBucketActive",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "currentBucketId",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "currentBucketStartTimestamp",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "currentBucketEndTimestamp",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "currentBucketCapacity",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "currentBucketRateNumer",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "currentBucketRateDenom",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "currentBucketSoldAmount",
      }),
    ]);

    setCrowdsale({
      phaseContract: {
        name,
        isPhaseActive,
        isBucketActive,
        // bucket info
        currentBucketId,
        currentBucketStartTimestamp,
        currentBucketEndTimestamp,
        currentBucketCapacity,
        currentBucketRate:
          Number((currentBucketRateNumer * 100n) / currentBucketRateDenom) /
          100,
        currentBucketSoldAmount,
      },
    });

    unwatchers = [
      wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        eventName: "BucketCreated",
        onLogs: (logs) => {
          const event = logs.at(0)?.args;
          if (event) {
            setCrowdsale("phaseContract", () => ({
              isBucketActive: true,
              currentBucketId: event.bucketId,
              currentBucketStartTimestamp: event.startTimestamp,
              currentBucketEndTimestamp: event.endTimestamp,
              currentBucketCapacity: event.capacity,
              currentBucketRate:
                Number((event.rateNumer * 100n) / event.rateDenom) / 100,
            }));
          }
        },
      }),
      wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        eventName: "BucketConcluded",
        onLogs: (logs) => {
          const event = logs.at(0)?.args;
          if (event) {
            setCrowdsale("phaseContract", () => ({
              isBucketActive: false,
              id: event.bucketId,
            }));
            setCrowdsale({ timer: undefined });
          }
        },
      }),
      wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        eventName: "BuyExecuted",
        onLogs: (logs) => {
          const event = logs.at(0)?.args;
          if (event) {
            setCrowdsale("phaseContract", () => ({
              currentBucketSoldAmount: event.bucketSoldAmount,
            }));
          }
        },
      }),
      wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        eventName: "PhaseConcluded",
        onLogs: (logs) => {
          const event = logs.at(0)?.args;
          if (event) {
            setCrowdsale("phaseContract", () => ({
              isPhaseActive: false,
              isBucketActive: false,
            }));
          }
        },
      }),
    ];
  });

  onCleanup(() => {
    unwatchers.forEach((unwatcher) => unwatcher());
  });

  return (
    <CrowdsaleContext.Provider value={crowdsale}>
      {props.children}
    </CrowdsaleContext.Provider>
  );
}
export function useCrowdsale() {
  return useContext<CrowdsaleProvider>(CrowdsaleContext);
}
