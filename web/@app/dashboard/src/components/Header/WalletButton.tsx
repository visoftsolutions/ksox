import { joinPaths } from "solid-start/islands/server-router";
import { api, base } from "~/root";
import { setSession, login, useSession, logout } from "@web/components/providers/SessionProvider";
import { useWallet, walletClientConnect } from "@web/components/providers/WalletProvider";
import firstLastChars from "@web/utils/firstLastChars";
import { createEffect, untrack } from "solid-js";

export default function WalletButton() {
  const wallet = useWallet();
  const session = useSession();

  createEffect(async () => {
    if (wallet.walletClient && untrack(() => !session())) {
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
        } else if (session()) {
          setSession(await logout(api));
        } else {
          setSession(await login(api, wallet.walletClient));
        }
      }}
    >
      {!session() ? (
        <img src={joinPaths(base, "gfx/wallet.svg")} alt="wallet" width="22px" class="m-auto" />
      ) : (
        <img src={joinPaths(base, "gfx/user.svg")} alt="user" width="16px" class="m-auto" />
      )}
      <div class="text-ellipsis text-wallet font-semibold">
        {!wallet.walletClient && !session() ? "CONNECT WALLET" : !session() ? "LOGIN" : firstLastChars(session()?.user_id ?? "", 6, 6)}
      </div>
    </div>
  );
}
