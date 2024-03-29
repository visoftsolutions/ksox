import { A } from "solid-start";
import { joinPaths } from "solid-start/islands/server-router";
import { api, base } from "~/root";
import WalletButton from "~/components/Header/WalletButton";

export default function Header() {
  return (
    <div class="grid h-full grid-cols-[1fr_auto] items-center justify-center bg-gray-2 px-4">
      <A href="/">
        <div class="col-start-1 col-end-2 grid grid-cols-[auto_1fr] items-center justify-center gap-4 font-lexend text-lg font-bold">
          <div class="col-start-1 col-end-2 py-4">
            <img
              src={joinPaths(base, "/gfx/logo.svg")}
              alt="ksox logo"
              class="m-auto h-[47px] w-[36px]"
            />
          </div>
          <div class="col-start-2 col-end-3 max-[400px]:hidden">KSOX PAY</div>
        </div>
      </A>
      <div class="col-start-2 col-end-3">
        <WalletButton base_url={base} api_url={api} />
      </div>
    </div>
  );
}
