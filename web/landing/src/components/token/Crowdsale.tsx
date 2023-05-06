import { createSignal } from "solid-js";
import Divider from "../Divider";
import ProgressBar from "./ProgressBar";
import TimerTile from "./TimerTile";

interface CrowdsaleProps {
    phaseName: string; // eq Phase 1
    status: boolean; // active / inactive
}

export default function Crowdsale (props: CrowdsaleProps) {
    const [timerDays, setTimerDays] = createSignal<string | undefined>(undefined);
    const [timerHours, setTimerHours] = createSignal<string | undefined>(undefined);
    const [timerMinutes, setTimerMinutes] = createSignal<string | undefined>(undefined);
    const [timerSeconds, setTimerSeconds] = createSignal<string | undefined>(undefined);
    const [openBucketNo, setOpenBucketNo] = createSignal<string | undefined>(undefined);
    const [totalBucketNo, setTotalBucketNo] = createSignal<string | undefined>(undefined);

    return (
        <div class="grid grid-rows-[auto_auto] font-lexend text-text-1 gap-24">
            <div class="row-start-1 row-end-2 text-center font-medium">
                <div class="text-4xl">KSXT Crowdsale</div>
                <div class="text-xl">{props.phaseName} - {props.status ? "active" : "inactive"}</div>
            </div>

            <div class={`grid grid-cols-[1fr_1fr] row-start-2 row-end-3 gap-5 gap-y-16 items-stretch max-md:grid-cols-1 max-md:grid-rows-2 ${props.status ? "text-text-1" : "text-gray-700"}`}>
                
                <div class="col-start-1 col-end-2 max-md:col-start-1 max-md:col-end-2 max-md:row-start-2 max-md:row-end-3 grid grid-rows-[auto_1fr_80px] gap-8 px-16 max-md:px-1 justify-items-stretch">
                    <div class="row-start-1 row-end-2 text-2xl font-medium text-center px-8">
                        SALE ENDS IN
                    </div>

                    <div class={`row-start-2 row-end-3 text-2xl font-medium grid grid-cols-4 justify-around items-center gap-4 text-center max-md:gap-4 `}>
                        <TimerTile name="days" value={timerDays() ?? "X"}/>
                        <TimerTile name="hrs" value={timerHours() ?? "X"}/>
                        <TimerTile name="mins" value={timerMinutes() ?? "X"}/>
                        <TimerTile name="secs" value={timerSeconds() ?? "X"}/>
                    </div>

                    <div class="row-start-3 row-end-4 self-end grid grid-rows-[auto_auto] gap-1">
                        <div>Phase supply</div>
                        <ProgressBar fill={0.1} disable={!props.status}/>
                    </div>
                </div>

                <div class="col-start-2 col-end-3 max-md:col-start-1 max-md:col-end-2 max-md:row-start-1 max-md:row-end-2 grid grid-rows-[auto_auto_1fr_80px] gap-8 px-16 max-md:px-1 justify-items-stretch">
                    <div class="row-start-1 row-end-2 text-2xl font-medium text-center px-8">
                        OPEN BUCKET {openBucketNo() ?? "X"}/{totalBucketNo() ?? "X"}
                    </div>

                    <div class="row-start-2 row-end-3 self-center text-lg font-medium">
                        KSXT Token Price = 0.10$
                    </div>

                    <div class="row-start-3 row-end-4 self-center grid grid-rows-[auto_auto] gap-1">
                        <div>Bucket supply</div>
                        <ProgressBar fill={0.8} disable={!props.status}/>
                    </div>

                    <div class="row-start-4 row-end-5 self-end">
                        <div class={`rounded-full p-[11px_32px] text-center font-lexend text-hero-button font-medium md:p-[16px_40px] ${props.status ? "text-text-1 cursor-pointer hover:opacity-90 transition-opacity duration-100 token-linear-wipe-button" : "text-gray-700 bg-gray-900"}`}>
                            Buy KSXT Token
                        </div>
                    </div>
                </div>


            </div>

            <Divider />

            <div class="grid grid-cols-[1fr_1fr] max-md:grid-cols-[1fr]">
                <div class="col-start-1 col-end-2 grid grid-rows-[auto_1fr] gap-4">
                    <div class="text-4xl font-medium">The Bucket System</div>
                    <div class="pl-5 text-xl">
                        <div class="py-1">Every day at 00:00 UTC, a new bucket is created.</div>
                        <div class="py-1">Each bucket has a predetermined, fixed capacity.</div>
                        <div class="py-1">Purchasing more than 100,000 tokens overrides the capacity limitations of the bucket.</div>
                        <div class="py-1">Checkout <a href="/whitepaper/ksox-whitepaper.pdf#Tokenomics" target="_blank" class="text-links_new">Tokenomics</a> section of <a href="/whitepaper/ksox-whitepaper.pdf" target="_blank" class="text-links_new">Whitepaper</a> to learn more.</div>
                    </div>
                </div>
                <div class="col-start-2 col-end-3 max-md:hidden">
                    
                </div>

            </div>

            <Divider />
        </div>
    );
}