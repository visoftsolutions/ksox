import { createContext, JSX, onMount, useContext } from "solid-js";
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

const CrowdsaleContext = createContext<CrowdsaleProvider>(crowdsale);
export function CrowdsaleProvider(props: { children: JSX.Element }) {
  const wallet = useWallet();

  onMount(async () => {
    try {
      const name = await wallet.publicClient.readContract({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
        functionName: "name",
      });
      const isPhaseActive = await wallet.publicClient.readContract({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
        functionName: "isPhaseActive",
      });
      const isBucketActive = await wallet.publicClient.readContract({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
        functionName: "isBucketActive",
      });
      const currentBucketId = await wallet.publicClient.readContract({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
        functionName: "currentBucketId",
      });
      const currentBucketStartTimestamp =
        await wallet.publicClient.readContract({
          address: wallet.selected_network.phaseContract.address,
          abi: wallet.selected_network.phaseContract.abi,
          functionName: "currentBucketStartTimestamp",
        });
      const currentBucketEndTimestamp = await wallet.publicClient.readContract({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
        functionName: "currentBucketEndTimestamp",
      });
      const currentBucketCapacity = await wallet.publicClient.readContract({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
        functionName: "currentBucketCapacity",
      });
      const currentBucketRateNumer = await wallet.publicClient.readContract({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
        functionName: "currentBucketRateNumer",
      });
      const currentBucketRateDenom = await wallet.publicClient.readContract({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
        functionName: "currentBucketRateDenom",
      });
      const currentBucketSoldAmount = await wallet.publicClient.readContract({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
        functionName: "currentBucketSoldAmount",
      });

      setCrowdsale("phaseContract", () => ({
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
      }));

      wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
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
              currentBucketSoldAmount: 0n,
              currentBucketRate:
                Number((event.rateNumer * 100n) / event.rateDenom) / 100,
            }));
          }
        },
      });
      wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
        eventName: "BucketConcluded",
        onLogs: (logs) => {
          const event = logs.at(0)?.args;
          if (event) {
            setCrowdsale("phaseContract", () => ({
              isBucketActive: false,
              currentBucketId: event.bucketId,
            }));
          }
        },
      });
      wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
        eventName: "BuyExecuted",
        onLogs: (logs) => {
          const event = logs.at(0)?.args;
          if (event) {
            setCrowdsale("phaseContract", () => ({
              currentBucketSoldAmount: event.bucketSoldAmount,
            }));
          }
        },
      });
      wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.phaseContract.address,
        abi: wallet.selected_network.phaseContract.abi,
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
      });
    } catch (error) {
      console.log("Communication with smart-contract failed");
    }
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
