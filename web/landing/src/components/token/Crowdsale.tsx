import Spacing from "../Spacing";
import Bucket from "./Bucket";

export default function Crowdsale () {
    return (
        <div class="grid grid-rows-[auto_1fr_auto] font-lexend text-text-1">
            <div class="grid gap-2 grid-rows-2 row-start-1 row-end-2 text-center text-4xl font-medium">
                <div class="row-start-1 row-end-2">KSXT Crowdsale</div>
                {/* <div class="row-start-2 row-end-3 text-sm font-light">
                    <div>KSXT Ethereum Address</div>
                    <div>0x3acaDFB15E991e8403D2Fe3E75Ee4782B88cF5b1</div>
                </div> */}
                 <Spacing class="h-16" />
            </div>

            <div class="row-start-2 row-end-3">
                <div class="grid grid-flow-col gap-[1px] h-[200px]">
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
                    <Bucket fill={0.0}/>
                </div>
                
                <Spacing class="h-16" />

                <div class="text-2xl font-medium grid grid-flow-col justify-center items-center gap-2">
                    <div class="p-1">Bucket</div>
                    <div class="p-1">#{21}</div>
                    <div class="p-1 mx-5 text-4xl">{(Math.floor(0.42*10000)/100).toFixed(2)}%</div>
                    <div class="p-1 text-lg">closes in 1h : 10m : 12s</div>
                </div>

                <Spacing class="h-16" />
            </div>
            <div class="row-start-3 row-end-4 rounded-full token-linear-wipe-button p-[11px_32px] text-center font-lexend text-hero-button font-medium text-text-1 md:p-[16px_40px]">
                Buy KSXT Token
            </div>
        </div>
    );
}