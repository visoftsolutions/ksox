import Divider from "../Divider";
import ProgressBar from "./ProgressBar";
import TimerTile from "./TimerTile";

export default function Crowdsale () {
    return (
        <div class="grid grid-rows-[auto_auto] font-lexend text-text-1 gap-24">
            <div class="row-start-1 row-end-2 text-center font-medium">
                <div class="text-4xl">KSXT Crowdsale</div>
                <div class="text-xl">Phase 1</div>
            </div>

            <div class="grid grid-cols-[1fr_1fr] row-start-2 row-end-3 gap-5 items-stretch">
                
                <div class="col-start-1 col-end-2 grid grid-rows-[auto_1fr_80px] gap-8 px-16 justify-items-stretch">
                    <div class="row-start-1 row-end-2 text-2xl font-medium text-center px-8">
                        SALE ENDS IN
                    </div>

                    <div class="row-start-2 row-end-3 text-2xl font-medium grid grid-cols-4 justify-around items-center gap-4 text-center">
                        <TimerTile name="days" value="12"/>
                        <TimerTile name="hours" value="01"/>
                        <TimerTile name="minutes" value="10"/>
                        <TimerTile name="seconds" value="03"/>
                    </div>

                    <div class="row-start-3 row-end-4 self-end grid grid-rows-[auto_auto] gap-1">
                        <div>Phase supply</div>
                        <ProgressBar fill={0.1}/>
                    </div>
                </div>

                <div class="col-start-2 col-end-3 grid grid-rows-[auto_auto_1fr_80px] gap-8 px-16 justify-items-stretch">
                    <div class="row-start-1 row-end-2 text-2xl font-medium text-center px-8">
                        OPEN BUCKET 7/32
                    </div>

                    <div class="row-start-2 row-end-3 self-center text-lg font-medium">
                        KSXT Token Price = 0.10$
                    </div>

                    <div class="row-start-3 row-end-4 self-center grid grid-rows-[auto_auto] gap-1">
                        <div>Bucket supply</div>
                        <ProgressBar fill={0.8}/>
                    </div>

                    <div class="row-start-4 row-end-5 self-end">
                        <div class="rounded-full token-linear-wipe-button p-[11px_32px] text-center font-lexend text-hero-button font-medium text-text-1 md:p-[16px_40px] cursor-pointer hover:opacity-90 transition-opacity duration-100">
                            Buy KSXT Token
                        </div>
                    </div>
                </div>


            </div>

            <Divider />

            <div class="grid grid-cols-[1fr_1fr]">
                <div class="col-start-1 col-end-2 grid grid-rows-[auto_1fr] gap-4">
                    <div class="text-4xl font-medium">The Bucket System</div>
                    <div class="pl-5 text-xl">
                        <div class="py-1">Every day at 00:00 UTC, a new bucket is created.</div>
                        <div class="py-1">Each bucket has a predetermined, fixed capacity.</div>
                        <div class="py-1">Purchasing more than 100,000 tokens overrides the capacity limitations of the bucket.</div>
                        <div class="py-1">Checkout <a href="/whitepaper/ksox-whitepaper.pdf#Tokenomics" target="_blank" class="text-links_new">Tokenomics</a> section of <a href="/whitepaper/ksox-whitepaper.pdf" target="_blank" class="text-links_new">Whitepaper</a> to learn more.</div>
                    </div>
                </div>
                <div class="col-start-2 col-end-3">
                    
                </div>

            </div>

            <Divider />
        </div>
    );
}