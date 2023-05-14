import { createContext, JSX, onCleanup, onMount, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { useWallet } from "./WalletProvider";

export interface PhaseContract {
  phaseName: string;
  isPhaseActive: boolean;
  isBucketActive: boolean;
  // bucket info
  id: bigint;
  startTimestamp: bigint;
  endTimestamp: bigint;
  capacity: bigint;
  soldAmount: bigint;
  rate: number;
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
    phaseName: "",
    isPhaseActive: false,
    isBucketActive: false,
    // bucket info
    id: 0n,
    startTimestamp: 0n,
    endTimestamp: 0n,
    capacity: 0n,
    soldAmount: 0n,
    rate: 0,
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

  let unwatchers: unwatch[] = []

  onMount(async () => {

    const [
      phaseName,
      isPhaseActive,
      isBucketActive,
      id,
      startTimestamp,
      endTimestamp,
      capacity,
      soldAmount,
      rateNumer,
      rateDenom,
    ] = await Promise.all([
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "phaseName",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "isPhaseActive",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "id",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "startTimestamp",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "endTimestamp",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "capacity",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "soldAmount",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "rateNumer",
      }),
      wallet.publicClient.readContract({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        functionName: "rateDenom",
      }),
    ]);

    setCrowdsale({
      phaseContract: {
        phaseName,
        isPhaseActive,
        isBucketActive,
        // bucket info
        id,
        startTimestamp,
        endTimestamp,
        capacity,
        soldAmount,
        rate: Number(rateNumer * 100n / rateDenom) / 100
      },
    });

    unwatchers = [
      wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        eventName: 'BucketCreated',
        onLogs: (logs) => {
          const event = logs.at(0)?.args
          if (event) {
            setCrowdsale('phaseContract', _p => ({
              isBucketActive: true,
              id: event.id,
              startTimestamp: event.startTimestamp,
              endTimestamp: event.endTimestamp,
              capacity: event.capacity,
              rate: Number(event.rateNumer * 100n / event.rateDenom) / 100
            }))
          }
        },
      }),
      wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        eventName: 'BucketConcluded',
        onLogs: (logs) => {
          const event = logs.at(0)?.args
          if (event) {
            setCrowdsale('phaseContract', _p => ({
              isBucketActive: false,
              id: event.id,
            }));
            setCrowdsale({timer: undefined})
          }
        },
      }),
      wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        eventName: 'BuyExecuted',
        onLogs: (logs) => {
          const event = logs.at(0)?.args
          if (event) {
            setCrowdsale('phaseContract', p => ({
              soldAmount: event.bucketSoldAmount
            }))
          }
        },
      }),
      wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        eventName: 'PhaseConcluded',
        onLogs: (logs) => {
          const event = logs.at(0)?.args
          if (event) {
            setCrowdsale('phaseContract', _p => ({
              isPhaseActive: false,
              isBucketActive: false,
            }))
          }
        },
      })
    ]

  })

  onCleanup(() => {
    unwatchers.forEach(unwatcher => unwatcher());
  }) 

  return (
    <CrowdsaleContext.Provider value={crowdsale}>
      {props.children}
    </CrowdsaleContext.Provider>
  );
}
export function useCrowdsale() {
  return useContext<CrowdsaleProvider>(CrowdsaleContext);
}
