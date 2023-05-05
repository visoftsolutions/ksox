import Spacing from "../Spacing";
import Bucket from "./Bucket";
import ProgressBar from "./ProgressBar";
import TimerTile from "./TimerTile";

export default function Crowdsale () {
    return (
        <div class="grid grid-rows-[auto_auto_auto] font-lexend text-text-1 gap-24">
            <div class="row-start-1 row-end-2 text-center text-4xl font-medium">KSXT Crowdsale</div>

            <div class="grid grid-cols-[1fr_1fr] grid-rows-[auto_auto] row-start-2 row-end-3 gap-5 gap-y-16">
                <div class="col-start-1 col-end-2 row-start-1 row-end-2 grid grid-flow-col gap-[1px]">
                        <Bucket fill={0.1}/>
                        <Bucket fill={0.2}/>
                        <Bucket fill={0.4}/>
                        <Bucket fill={0.1}/>
                        <Bucket fill={0.2}/>
                        <Bucket fill={0.1}/>
                        <Bucket fill={0.2}/>
                        <Bucket fill={1}/>
                        <Bucket fill={0.1}/>
                        <Bucket fill={0.2}/>
                        <Bucket fill={1}/>
                        <Bucket fill={0.2}/>
                        <Bucket fill={0.4}/>
                        <Bucket fill={0.1}/>
                        <Bucket fill={0.2}/>
                        <Bucket fill={0.1}/>
                        <Bucket fill={0.2}/>
                        <Bucket fill={0.4}/>
                        <Bucket fill={0.42}/>
                        <Bucket fill={0.7}/>
                        <Bucket fill={0.0}/>
                        <Bucket fill={0.0}/>
                        <Bucket fill={0.0}/>
                        <Bucket fill={0.0}/>
                        <Bucket fill={0.0}/>
                        <Bucket fill={0.0}/>
                        <Bucket fill={0.0}/>
                        <Bucket fill={0.0}/>
                        <Bucket fill={0.0}/>
                        <Bucket fill={0.0}/>
                </div>
                
                <div class="col-start-2 col-end-3 row-start-1 row-end-2 text-2xl font-medium grid grid-rows-2 justify-center items-center px-16">
                    <div class="text-center">SALE ENDS IN</div>
                    <div class="grid grid-cols-4 justify-around items-center gap-4">
                        <TimerTile name="days" value="12"/>
                        <TimerTile name="hours" value="01"/>
                        <TimerTile name="minutes" value="10"/>
                        <TimerTile name="seconds" value="03"/>
                    </div>
                </div>

                <div class="col-start-1 col-end-2 row-start-2 row-end-3 grid items-center justify-center self-center">
                    <div class="grid grid-flow-col justify-center items-center">
                        <div class="p-1">Bucket #{21} filled in:</div>
                        <div class="p-1 ml-5 text-4xl">{(Math.floor(0.42*10000)/100).toFixed(2)}%</div>
                    </div>
                </div>

                <div class="col-start-2 col-end-3 row-start-2 row-end-3 self-center">
                    <div class="px-16">
                        <ProgressBar fill={0.8}/>
                    </div>
                </div>
            </div>
            <div class="row-start-3 row-end-4 rounded-full token-linear-wipe-button p-[11px_32px] text-center font-lexend text-hero-button font-medium text-text-1 md:p-[16px_40px]">
                Buy KSXT Token
            </div>
        </div>
    );
}