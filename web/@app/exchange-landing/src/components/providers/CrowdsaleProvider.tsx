import { useWallet } from "@web/components/providers/WalletProvider";
import { createContext, createEffect, JSX, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { mainnet, bsc, polygon } from "@wagmi/chains";
import { Address } from "viem";
import { PHASE_CONTRACT_ABI } from "~/contracts/abi/phaseContract";
import { WETH_CONTRACT_ABI } from "~/contracts/abi/wethContract";
import { TOKEN_TICKET_CONTRACT_ABI } from "~/contracts/abi/tokenTicketContract";

export interface Token {
  icon: string;
  name: string;
  symbol: string;
}

export interface phaseContract {
  address: Address;
  abi: typeof PHASE_CONTRACT_ABI;
}

export interface wethContract {
  address: Address;
  abi: typeof WETH_CONTRACT_ABI;
}

export interface tokenTicketContract {
  address: Address;
  abi: typeof TOKEN_TICKET_CONTRACT_ABI;
}

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
  selected_token: Token;
  phaseContract: PhaseContract;
  showModal: boolean;
  timer: Timer;
  tokenAmount: number;
}

export interface Contract {
  tokens: Token[];
  phaseContract: phaseContract;
  wethContract: wethContract;
  tokenTicketContract: tokenTicketContract;
}

export const CONTRACT_ON_CHAIN: Map<string, Contract> = new Map([
  [
    mainnet.name,
    {
      phaseContract: {
        address: "0x0000000000000000000000000000000000000000",
        abi: PHASE_CONTRACT_ABI,
      },
      wethContract: {
        address: "0x0000000000000000000000000000000000000000",
        abi: WETH_CONTRACT_ABI,
      },
      tokenTicketContract: {
        address: "0x0000000000000000000000000000000000000000",
        abi: TOKEN_TICKET_CONTRACT_ABI,
      },
      tokens: [
        { icon: "/gfx/asset_icons/eth.svg", name: "Ethereum", symbol: "ETH" },
        {
          icon: "/gfx/asset_icons/eth.svg",
          name: "Wrapped Ethereum",
          symbol: "WETH",
        },
        { icon: "/gfx/asset_icons/usdc.svg", name: "USD Coin", symbol: "USDC" },
        { icon: "/gfx/asset_icons/usdt.svg", name: "Tether", symbol: "USDT" },
      ],
    },
  ],
  [
    polygon.name,
    {
      phaseContract: {
        address: "0x0000000000000000000000000000000000000000",
        abi: PHASE_CONTRACT_ABI,
      },
      wethContract: {
        address: "0x0000000000000000000000000000000000000000",
        abi: WETH_CONTRACT_ABI,
      },
      tokenTicketContract: {
        address: "0x0000000000000000000000000000000000000000",
        abi: TOKEN_TICKET_CONTRACT_ABI,
      },
      tokens: [
        { icon: "/gfx/asset_icons/eth.svg", name: "Ethereum", symbol: "ETH" },
        {
          icon: "/gfx/asset_icons/eth.svg",
          name: "Wrapped Ethereum",
          symbol: "WETH",
        },
        { icon: "/gfx/asset_icons/usdc.svg", name: "USD Coin", symbol: "USDC" },
        { icon: "/gfx/asset_icons/usdt.svg", name: "Tether", symbol: "USDT" },
      ],
    },
  ],
  [
    bsc.name,
    {
      phaseContract: {
        address: "0x0000000000000000000000000000000000000000",
        abi: PHASE_CONTRACT_ABI,
      },
      wethContract: {
        address: "0x0000000000000000000000000000000000000000",
        abi: WETH_CONTRACT_ABI,
      },
      tokenTicketContract: {
        address: "0x0000000000000000000000000000000000000000",
        abi: TOKEN_TICKET_CONTRACT_ABI,
      },
      tokens: [
        { icon: "/gfx/asset_icons/eth.svg", name: "Ethereum", symbol: "ETH" },
        {
          icon: "/gfx/asset_icons/eth.svg",
          name: "Wrapped Ethereum",
          symbol: "WETH",
        },
        { icon: "/gfx/asset_icons/usdc.svg", name: "USD Coin", symbol: "USDC" },
        { icon: "/gfx/asset_icons/usdt.svg", name: "Tether", symbol: "USDT" },
      ],
    },
  ],
]);

export const [crowdsale, setCrowdsale] = createStore<CrowdsaleProvider>({
  selected_token: { icon: "/gfx/asset_icons/eth.svg", name: "Ethereum", symbol: "ETH" },
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

  createEffect(async () => {
    const publicClient = wallet.publicClient;
    const publicWSClient = wallet.publicWSClient;
    const phaseContract = CONTRACT_ON_CHAIN.get(wallet.selected_network.network.name)?.phaseContract;

    if (phaseContract && publicClient && publicWSClient) {
      try {
        const name = await publicClient.readContract({
          address: phaseContract.address,
          abi: phaseContract.abi,
          functionName: "name",
        });
        const isPhaseActive = await publicClient.readContract({
          address: phaseContract.address,
          abi: phaseContract.abi,
          functionName: "isPhaseActive",
        });
        const isBucketActive = await publicClient.readContract({
          address: phaseContract.address,
          abi: phaseContract.abi,
          functionName: "isBucketActive",
        });
        const currentBucketId = await publicClient.readContract({
          address: phaseContract.address,
          abi: phaseContract.abi,
          functionName: "currentBucketId",
        });
        const currentBucketStartTimestamp = await publicClient.readContract({
          address: phaseContract.address,
          abi: phaseContract.abi,
          functionName: "currentBucketStartTimestamp",
        });
        const currentBucketEndTimestamp = await publicClient.readContract({
          address: phaseContract.address,
          abi: phaseContract.abi,
          functionName: "currentBucketEndTimestamp",
        });
        const currentBucketCapacity = await publicClient.readContract({
          address: phaseContract.address,
          abi: phaseContract.abi,
          functionName: "currentBucketCapacity",
        });
        const currentBucketRateNumer = await publicClient.readContract({
          address: phaseContract.address,
          abi: phaseContract.abi,
          functionName: "currentBucketRateNumer",
        });
        const currentBucketRateDenom = await publicClient.readContract({
          address: phaseContract.address,
          abi: phaseContract.abi,
          functionName: "currentBucketRateDenom",
        });
        const currentBucketSoldAmount = await publicClient.readContract({
          address: phaseContract.address,
          abi: phaseContract.abi,
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
          currentBucketRate: Number((currentBucketRateNumer * 100n) / currentBucketRateDenom) / 100,
          currentBucketSoldAmount,
        }));

        publicWSClient.watchContractEvent({
          address: phaseContract.address,
          abi: phaseContract.abi,
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
                currentBucketRate: Number(((event.rateNumer ?? 0n) * 100n) / (event.rateDenom ?? 0n)) / 100,
              }));
            }
          },
        });
        publicWSClient.watchContractEvent({
          address: phaseContract.address,
          abi: phaseContract.abi,
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
        publicWSClient.watchContractEvent({
          address: phaseContract.address,
          abi: phaseContract.abi,
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
        publicWSClient.watchContractEvent({
          address: phaseContract.address,
          abi: phaseContract.abi,
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
    }
  });

  return <CrowdsaleContext.Provider value={crowdsale}>{props.children}</CrowdsaleContext.Provider>;
}
export function useCrowdsale() {
  return useContext<CrowdsaleProvider>(CrowdsaleContext);
}
