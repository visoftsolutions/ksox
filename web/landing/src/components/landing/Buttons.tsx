import { A } from "solid-start";
export default function Buttons() {
  return (
    <div class="grid grid-cols-2 grid-rows-2 gap-6 max-md:grid-rows-3 max-md:grid-cols-1">
      <a href="/app" class="col-start-1 col-end-2 row-start-1 row-end-2 max-md:col-start-1 max-md:col-end-2 max-md:row-start-1 max-md:row-end-2">
        <div class="rounded-full bg-primary p-[11px_32px] text-center font-lexend text-hero-button font-medium text-text-1 hover:bg-text-1 hover:text-buttonbg_new md:p-[16px_40px]">
          Launch App
        </div>
      </a>
      <A href="#section-1" class="col-start-2 col-end-3 row-start-1 row-end-2 max-md:col-start-1 max-md:col-end-2 max-md:row-start-2 max-md:row-end-3">
        <div class="rounded-full border-2 border-solid border-text-1 p-[10px_32px] text-center font-lexend text-hero-button font-medium text-text-1 hover:bg-buttonbg_new md:p-[16px_40px]">
          Learn More
        </div>
      </A>
      <A href="/token" class="col-start-1 col-end-3 row-start-2 row-end-3 max-md:col-start-1 max-md:col-end-2 max-md:row-start-3 max-md:row-end-4">
        <div class="rounded-full token-linear-wipe-button p-[11px_32px] text-center font-lexend text-hero-button font-medium text-text-1 md:p-[16px_40px]">
          Buy Token
        </div>
      </A>
    </div>
  );
}
