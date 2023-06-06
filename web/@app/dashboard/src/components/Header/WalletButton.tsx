import { joinPaths } from "solid-start/islands/server-router";
import { api, base } from "~/root";
import { setSession, login, useSession } from "@web/components/providers/SessionProvider";
import { walletClientConnect, useWallet } from "@web/components/providers/WalletProvider";
import firstLastChars from "@web/utils/firstLastChars";
import { createEffect } from "solid-js";

export default function WalletButton() {
  const wallet = useWallet();
  const session = useSession();

  createEffect(async () => {
    if (wallet.walletClient) {
      setSession(await login(api, wallet.walletClient));
    }
  });

  return (
    <div
      class={`grid h-[32px] cursor-pointer select-none grid-cols-[auto_1fr] items-center justify-center gap-2 rounded-[40px] px-5 ${
        !wallet || !session() ? "bg-ksox-1" : "bg-gray-3"
      }`}
      onClick={async () => {
        if (!wallet.walletClient) {
          await walletClientConnect();
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
