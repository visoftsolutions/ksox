import Divider from "../Divider";
import ProgressBar from "./ProgressBar";
import TimerTile from "./TimerTile";
import {
  setCrowdsale,
  useCrowdsale,
} from "~/utils/providers/CrowdsaleProvider";
import TokenDropdown from "./TokenDropdown";
import AmountInput from "./AmountInput";
import { useWallet } from "~/utils/providers/WalletProvider";
import { createEffect, createSignal, onCleanup, onMount } from "solid-js";
import { ev, fFromBigint, finv, fmul } from "../types/primitives/fraction";

export default function Crowdsale() {
  const crowdsale = useCrowdsale();
  const wallet = useWallet();

  const [nowTimestamp, setNowTimestamp] = createSignal<number>(Math.floor(Date.now()/1000));

  createEffect(async () => {
    if (
      wallet.publicClient != undefined &&
      wallet.publicWSClient != undefined &&
      wallet.selected_network != undefined
    ) {
      const unwatchNewBucketCreated = wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        eventName: 'NewBucketCreated',
        onLogs: (logs) => {
          const event = logs.at(0)?.args
          if (event) {
            setCrowdsale('phaseState', _p => ({
              isBucketActive: true,
              currentBucketId: event.bucketId,
              bucketStartTimestamp: event.startTimestamp,
              bucketFinishTimestamp: event.finishTimestamp,
              currentBucketAmountToSell: event.amountToSell,
              rate: Number(event.newRateNumer) / Number(event.newRateDenom),
            }))
          }
        },
      });

      const unwatchBucketConcluded = wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        eventName: 'BucketConcluded',
        onLogs: (logs) => {
          const event = logs.at(0)?.args
          if (event) {
            setCrowdsale('phaseState', _p => ({
              isBucketActive: false
            }));
            setCrowdsale({timer: undefined})
          }
        },
      });

      const unwatchBuyExecuted = wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        eventName: 'BuyExecuted',
        onLogs: (logs) => {
          const event = logs.at(0)?.args
          if (event) {
            setCrowdsale('phaseState', p => ({
              isBucketActive: true,
              amountSold: p.amountSold + event.amountOut
            }))
          }
        },
      });

      const unwatchPhaseConcluded = wallet.publicWSClient.watchContractEvent({
        address: wallet.selected_network.contract.address,
        abi: wallet.selected_network.contract.abi,
        eventName: 'PhaseConcluded',
        onLogs: (logs) => {
          const event = logs.at(0)?.args
          if (event) {
            setCrowdsale('phaseState', _p => ({
              isPhaseActive: false,
            }))
          }
        },
      });

      const [
        isPhaseActive,
        bucketStartTimestamp,
        bucketFinishTimestamp,
        currentBucketId,
        currentBucketAmountToSell,
        isBucketActive,
        currentRateNumer,
        currentRateDenom,
        prevAmountSold,
        amountSold,
      ] = await Promise.all([
        wallet.publicClient.readContract({
          address: wallet.selected_network.contract.address,
          abi: wallet.selected_network.contract.abi,
          functionName: "isPhaseActive",
        }),
        wallet.publicClient.readContract({
          address: wallet.selected_network.contract.address,
          abi: wallet.selected_network.contract.abi,
          functionName: "bucketStartTimestamp",
        }),
        wallet.publicClient.readContract({
          address: wallet.selected_network.contract.address,
          abi: wallet.selected_network.contract.abi,
          functionName: "bucketFinishTimestamp",
        }),
        wallet.publicClient.readContract({
          address: wallet.selected_network.contract.address,
          abi: wallet.selected_network.contract.abi,
          functionName: "currentBucketId",
        }),
        wallet.publicClient.readContract({
          address: wallet.selected_network.contract.address,
          abi: wallet.selected_network.contract.abi,
          functionName: "currentBucketAmountToSell",
        }),
        wallet.publicClient.readContract({
          address: wallet.selected_network.contract.address,
          abi: wallet.selected_network.contract.abi,
          functionName: "isBucketActive",
        }),
        wallet.publicClient.readContract({
          address: wallet.selected_network.contract.address,
          abi: wallet.selected_network.contract.abi,
          functionName: "currentRateNumer",
        }),
        wallet.publicClient.readContract({
          address: wallet.selected_network.contract.address,
          abi: wallet.selected_network.contract.abi,
          functionName: "currentRateDenom",
        }),
        wallet.publicClient.readContract({
          address: wallet.selected_network.contract.address,
          abi: wallet.selected_network.contract.abi,
          functionName: "prevAmountSold",
        }),
        wallet.publicClient.readContract({
          address: wallet.selected_network.contract.address,
          abi: wallet.selected_network.contract.abi,
          functionName: "amountSold",
        }),
      ]);

      setCrowdsale({
        phaseState: {
          isPhaseActive: isPhaseActive,
          bucketStartTimestamp: bucketStartTimestamp,
          bucketFinishTimestamp: bucketFinishTimestamp,
          currentBucketId: currentBucketId,
          currentBucketAmountToSell: currentBucketAmountToSell,
          isBucketActive: isBucketActive,
          amountSold: amountSold,
          prevAmountSold: prevAmountSold,
          rate: Number(currentRateNumer) / Number(currentRateDenom),
        },
      });
    }
  });

  const setTimer = (direction: boolean, duration: number) => {
    setCrowdsale('timer', _p => ({
      direction,
      timerDays: Math.floor(duration / (60*60*24)),
      timerHours: Math.floor((duration % (60*60*24)) / (60*60)),
      timerMinutes: Math.floor((duration % (60*60)) / 60),
      timerSeconds: Math.floor(duration % 60),
    }))
  }

  createEffect(() => {
    if (crowdsale.phaseState.bucketStartTimestamp && crowdsale.phaseState.bucketFinishTimestamp && nowTimestamp()) {
      const now = nowTimestamp();
      const bucketStart = Number(crowdsale.phaseState.bucketStartTimestamp);
      const bucketFinish = Number(crowdsale.phaseState.bucketFinishTimestamp);
      
      if (now <= bucketStart) {
        setTimer(true,  bucketStart - now)
      } else if (bucketStart < now && now <= bucketFinish) {
        setTimer(false,  bucketFinish - now)
      } else {
        setCrowdsale('phaseState', _p => ({
          isBucketActive: false
        }));
        setCrowdsale({timer: undefined})
      }
    }
  })

  let interval: NodeJS.Timer;
  
  onMount(() => {
    interval = setInterval(() => {
      setNowTimestamp(Math.floor(Date.now()/1000));
    }, 1000)
  })

  onCleanup(() => {
    clearInterval(interval);
  })

  return (
    <div class="grid grid-rows-[auto_auto] gap-24 font-lexend text-text-1">
      <div class="row-start-1 row-end-2 grid grid-rows-[auto_auto] gap-2 text-center font-medium">
        <div class="text-5xl">KSXT Crowdsale</div>
        <div class="text-xl">
          {crowdsale.phaseName} -{" "}
          {crowdsale.phaseState.isPhaseActive ? "active" : "inactive"}
        </div>
      </div>

      <div
        class={`row-start-2 row-end-3 grid grid-cols-[1fr_1fr] items-stretch gap-5 gap-y-16 max-md:grid-cols-1 max-md:grid-rows-2 ${
          crowdsale.phaseState.isBucketActive ? "text-text-1" : "text-gray-700"
        }`}
      >
        <div class="col-start-1 col-end-2 grid grid-flow-col grid-rows-[auto_1fr] items-center gap-8 px-16 max-md:col-start-1 max-md:col-end-2 max-md:row-start-2 max-md:row-end-3 max-md:px-1">
          <div class="row-start-1 row-end-2 px-8 text-center text-2xl font-medium grid grid-cols-[auto_auto] items-center justify-center gap-2">
            {crowdsale.phaseState.isBucketActive ? 
            <>
              OPEN BUCKET ID
              <div class="text-3xl token-linear-wipe-text font-extrabold">
                {crowdsale.phaseState.currentBucketId.toString()}
              </div>
            </> 
            : <>NO OPEN BUCKET</>}
          </div>

          <div class="row-start-2 row-end-3 grid grid-rows-[auto_auto] gap-8">
            <div class="px-8 text-center text-2xl font-medium">
              SALE {crowdsale.timer?.direction ? "STARTS" : "ENDS"} IN
            </div>

            <div class="grid grid-cols-4 items-start justify-around gap-4 text-center text-2xl font-medium max-md:gap-4">
              <TimerTile name="days" value={crowdsale.timer?.timerDays.toString() ?? "X"} />
              <TimerTile name="hrs" value={crowdsale.timer?.timerHours.toString() ?? "X"} />
              <TimerTile name="mins" value={crowdsale.timer?.timerMinutes.toString() ?? "X"} />
              <TimerTile name="secs" value={crowdsale.timer?.timerSeconds.toString() ?? "X"} />
            </div>
          </div>
        </div>

        <div class="col-start-2 col-end-3 grid grid-rows-[auto_auto_1fr] justify-items-stretch gap-12 px-16 max-md:col-start-1 max-md:col-end-2 max-md:row-start-1 max-md:row-end-2 max-md:px-1">
          <div class="row-start-1 row-end-2 px-8 text-center text-2xl font-medium">
            KSXT Token Price = {crowdsale.phaseState.rate?.toFixed(2) ?? "X"} USDC
          </div>

          <div class="row-start-2 row-end-3 grid grid-rows-[auto_auto] gap-1 self-center">
            <div>Bucket supply</div>
            <ProgressBar
              fill={ev(fmul(fFromBigint(crowdsale.phaseState.amountSold - crowdsale.phaseState.prevAmountSold), finv(fFromBigint(crowdsale.phaseState.currentBucketAmountToSell))))}
              disable={!crowdsale.phaseState.isBucketActive}
            />
          </div>

          <div class="row-start-3 row-end-4 grid grid-rows-[auto_auto] items-center gap-4 self-end">
            <div class="grid grid-cols-[1fr_auto] items-stretch justify-center gap-2">
              <AmountInput
                disabled={!crowdsale.phaseState.isBucketActive}
                onInput={(n) => setCrowdsale({ tokenAmount: n })}
              />
              <TokenDropdown disabled={!crowdsale.phaseState.isBucketActive} />
            </div>

            <div
              class={`rounded-full p-[11px_32px] text-center font-lexend text-hero-button font-medium md:p-[16px_40px] ${
                crowdsale.phaseState.isBucketActive
                  ? "token-linear-wipe-button cursor-pointer text-text-1 transition-opacity duration-100 hover:opacity-90"
                  : "bg-gray-900 text-gray-700"
              }`}
              onClick={() => {
                setCrowdsale({ showModal: true });
              }}
            >
              Buy KSXT Token
            </div>
          </div>
        </div>
      </div>

      <Divider />

      <div class="grid grid-cols-[1fr_1fr] max-md:grid-cols-[1fr]">
        <div class="col-start-1 col-end-2 grid grid-rows-[auto_1fr] gap-4">
          <div class="text-4xl font-medium">The Bucket System</div>
          <div class="text-xl">
            <div class="py-1">100% public sale</div>
            <div class="py-1">No vesting, no pre-allocation</div>
            <div class="py-1">Token Ticker instantly tradable</div>
            <div class="py-1">
              Each bucket has a predetermined, fixed capacity.
            </div>
            <div class="py-1">
              Checkout{" "}
              <a
                href="/whitepaper/ksox-whitepaper.pdf#Tokenomics"
                target="_blank"
                class="text-links_new"
              >
                Tokenomics
              </a>{" "}
              section of{" "}
              <a
                href="/whitepaper/ksox-whitepaper.pdf"
                target="_blank"
                class="text-links_new"
              >
                Whitepaper
              </a>{" "}
              to learn more.
            </div>
          </div>
        </div>
        <div class="col-start-2 col-end-3 max-md:hidden"></div>
      </div>

      <Divider />
    </div>
  );
}
