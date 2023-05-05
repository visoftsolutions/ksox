import ProgressBar from "./ProgressBar";
import TimerTile from "./TimerTile";

export default function Crowdsale () {
    return (
        <div class="grid grid-rows-[auto_auto] font-lexend text-text-1 gap-24">
            <div class="row-start-1 row-end-2 text-center text-4xl font-medium">KSXT Crowdsale</div>

            <div class="grid grid-cols-[1fr_1fr] row-start-2 row-end-3 gap-5 items-stretch">
                
                <div class="col-start-1 col-end-2 grid grid-rows-[auto_1fr_100px] gap-8 px-16 justify-items-stretch">
                    <div class="row-start-1 row-end-2 text-2xl font-medium text-center px-8">
                        SALE ENDS IN
                    </div>

                    <div class="row-start-2 row-end-3 text-2xl font-medium grid grid-cols-4 justify-around items-center gap-4 text-center">
                        <TimerTile name="days" value="12"/>
                        <TimerTile name="hours" value="01"/>
                        <TimerTile name="minutes" value="10"/>
                        <TimerTile name="seconds" value="03"/>
                    </div>

                    <div class="row-start-3 row-end-4 self-center">
                        <ProgressBar fill={0.1}/>
                    </div>
                </div>

                <div class="col-start-2 col-end-3 grid grid-rows-[auto_1fr_100px] gap-8 px-16 justify-items-stretch">
                    <div class="row-start-1 row-end-2 text-2xl font-medium text-center px-8">
                        OPEN BUCKET 7/32
                    </div>

                    <div class="row-start-2 row-end-3 self-center">
                        <ProgressBar fill={0.8}/>
                    </div>

                    <div class="row-start-3 row-end-4 self-center">
                        <div class="rounded-full token-linear-wipe-button p-[11px_32px] text-center font-lexend text-hero-button font-medium text-text-1 md:p-[16px_40px]">
                            Buy KSXT Token
                        </div>
                    </div>
                </div>


            </div>
            
        </div>
    );
}