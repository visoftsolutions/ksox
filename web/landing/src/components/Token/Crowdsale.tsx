import { createSignal } from "solid-js";
import Divider from "../Divider";
import ProgressBar from "./ProgressBar";
import TimerTile from "./TimerTile";
import {
  setCrowdsale,
  useCrowdsale,
} from "~/utils/providers/CrowdsaleProvider";
import TokenDropdown from "./TokenDropdown";

export default function Crowdsale() {
  const crowdsale = useCrowdsale();

  return (
    <div class="grid grid-rows-[auto_auto] gap-24 font-lexend text-text-1">
      <div class="row-start-1 row-end-2 grid grid-rows-[auto_auto] text-center font-medium gap-2">
        <div class="text-5xl">KSXT Crowdsale</div>
        <div class="text-xl">
          {crowdsale.phaseName} - {crowdsale.status ? "active" : "inactive"}
        </div>
      </div>

      <div
        class={`row-start-2 row-end-3 grid grid-cols-[1fr_1fr] items-stretch gap-5 gap-y-16 max-md:grid-cols-1 max-md:grid-rows-2 ${
          crowdsale.status ? "text-text-1" : "text-gray-700"
        }`}
      >
        <div class="col-start-1 col-end-2 grid grid-rows-[auto_1fr_80px] justify-items-stretch gap-8 px-16 max-md:col-start-1 max-md:col-end-2 max-md:row-start-2 max-md:row-end-3 max-md:px-1">
          <div class="row-start-1 row-end-2 px-8 text-center text-2xl font-medium">
            SALE ENDS IN
          </div>

          <div class="row-start-2 row-end-3 grid grid-cols-4 items-center justify-around gap-4 text-center text-2xl font-medium max-md:gap-4">
            <TimerTile name="days" value={crowdsale.timerDays ?? "X"} />
            <TimerTile name="hrs" value={crowdsale.timerHours ?? "X"} />
            <TimerTile name="mins" value={crowdsale.timerMinutes ?? "X"} />
            <TimerTile name="secs" value={crowdsale.timerSeconds ?? "X"} />
          </div>

          <div class="row-start-3 row-end-4 grid grid-rows-[auto_auto] gap-1 self-end">
            <div>Phase supply</div>
            <ProgressBar
              fill={crowdsale.phaseSupply}
              disable={!crowdsale.status}
            />
          </div>
        </div>

        <div class="col-start-2 col-end-3 grid grid-rows-[auto_auto_1fr_80px] justify-items-stretch gap-8 px-16 max-md:col-start-1 max-md:col-end-2 max-md:row-start-1 max-md:row-end-2 max-md:px-1">
          <div class="row-start-1 row-end-2 px-8 text-center text-2xl font-medium">
            OPEN BUCKET {crowdsale.openBucketNo ?? "X"}/
            {crowdsale.totalBucketNo ?? "X"}
          </div>

          <div class="row-start-2 row-end-3 self-center text-lg font-medium">
            KSXT Token Price = 0.10$
          </div>

          <div class="row-start-3 row-end-4 grid grid-rows-[auto_auto] gap-1 self-center">
            <div>Bucket supply</div>
            <ProgressBar
              fill={crowdsale.bucketSupply}
              disable={!crowdsale.status}
            />
          </div>

          <div class="row-start-4 row-end-5 self-end grid grid-cols-[1fr_auto] gap-2 items-center justify-center">
            <div
              class={`rounded-full p-[11px_32px] text-center font-lexend text-hero-button font-medium md:p-[16px_40px] ${
                crowdsale.status
                  ? "token-linear-wipe-button cursor-pointer text-text-1 transition-opacity duration-100 hover:opacity-90"
                  : "bg-gray-900 text-gray-700"
              }`}
              onClick={() => {
                setCrowdsale({ showModal: true });
              }}
            >
              Buy KSXT Token
            </div>
            <div class="grid grid-cols-[auto_auto] gap-2 items-center justify-center">
              <div class="text-lg">with</div>
              <TokenDropdown />
            </div>
          </div>
        </div>
      </div>

      <Divider />

      <div class="grid grid-cols-[1fr_1fr] max-md:grid-cols-[1fr]">
        <div class="col-start-1 col-end-2 grid grid-rows-[auto_1fr] gap-4">
          <div class="text-4xl font-medium">The Bucket System</div>
          <div class="text-xl">
            <div class="py-1">
              The duration of each bucket is {crowdsale.bucketInfo?.durationDays ?? "X"} days {crowdsale.bucketInfo?.durationHours ?? "X"} hrs {crowdsale.bucketInfo?.durationMinutes ?? "X"} mins {crowdsale.bucketInfo?.durationSeconds ?? "X"} secs
            </div>
            <div class="py-1">
              Each bucket has a predetermined, fixed capacity.
            </div>
            <div class="py-1">
              Purchasing more than{" "}
              {crowdsale.bucketInfo?.overrideThreshold ?? "X"} tokens overrides
              the capacity limitations of the bucket.
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
