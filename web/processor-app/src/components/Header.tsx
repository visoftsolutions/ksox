import { A } from "solid-start";
import WalletButton from "./Header/WalletButton";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";

export default function Header() {
  return (
    <div class="grid h-full grid-cols-[1fr_auto] items-center justify-center bg-gray-2 px-8">
      <div class="col-start-1 col-end-2 grid grid-cols-[auto_1fr] items-center justify-center gap-4 font-lexend text-lg font-bold">
        <div class="col-start-1 col-end-2 py-4">
          <A href="/">
            <img src={joinPaths(base, "/gfx/logo.png")} alt="ksox logo" class="m-auto h-[47px] w-[36px]" />
          </A>
        </div>
        <div class="col-start-2 col-end-3 max-[400px]:hidden">KSOX PAY</div>
      </div>
      <div class="col-start-2 col-end-3">
        <WalletButton />
      </div>
    </div>
  );
}
