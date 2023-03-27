import { A } from "solid-start";

export default function LaunchAppButton() {
  return (
    <A href="/app">
      <div class="rounded-full bg-primary p-[11px_32px] text-center font-lexend text-hero-button font-medium text-text-1 hover:bg-text-1 hover:text-buttonbg_new md:p-[16px_40px]">
        Launch App
      </div>
    </A>
  );
}
