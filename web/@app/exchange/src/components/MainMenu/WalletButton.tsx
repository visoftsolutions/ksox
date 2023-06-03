import { joinPaths } from "solid-start/islands/server-router";
import { api, base, projectId } from "~/root";
import { session, setSession, login, logout } from "@web/components/providers/SessionProvider";
import { walletClientConnect, wallet } from "@web/components/providers/WalletProvider";
import firstLastChars from "@web/utils/firstLastChars";
import { createEffect } from "solid-js";

export default function WalletButton() {

  createEffect(async () => {
    if (wallet.walletClient) {
      setSession(await login(api, wallet.walletClient))
    }
  })

  return (
    <div
      class={`grid h-[32px] cursor-pointer select-none grid-cols-[auto_1fr] items-center justify-center gap-2 rounded-[40px] px-5 ${
        !wallet || !session() ? "bg-ksox-1" : "bg-gray-3"
      }`}
      onClick={async () => {
        if (!wallet.walletClient) {
          await walletClientConnect(projectId);
        }
      }}
    >
      {!session() ? (
        <img src={joinPaths(base, "gfx/wallet.svg")} alt="wallet" width="22px" class="m-auto" />
      ) : (
        <img src={joinPaths(base, "gfx/user.svg")} alt="user" width="16px" class="m-auto" />
      )}
      <div class="text-ellipsis text-wallet font-semibold">
        {!wallet.walletClient ? "CONNECT WALLET" : !session() ? "LOGIN" : firstLastChars(wallet.address ?? "", 6, 6)}
      </div>
    </div>
  );
}
