import { joinPaths } from "solid-start/islands/server-router";
import {
  setSession,
  login,
  useSession,
  logout,
} from "@web/components/providers/SessionProvider";
import {
  useWallet,
  walletClientConnect,
} from "@web/components/providers/WalletProvider";
import firstLastChars from "@web/utils/firstLastChars";
import { createEffect, untrack } from "solid-js";

export default function WalletButton(props: {
  base_url: string;
  api_url: string;
}) {
  const wallet = useWallet();
  const session = useSession();

  createEffect(() => {
    if (wallet.walletClient && untrack(() => !session())) {
      login(props.api_url, wallet.walletClient).then((r) => setSession(r));
    }
  });

  return (
    <div
      class={`grid h-[32px] cursor-pointer select-none grid-cols-[auto_1fr] items-center justify-center gap-2 rounded-[40px] px-5 ${
        !wallet || !session() ? "bg-ksox-1" : "bg-gray-3"
      }`}
      onClick={() => {
        if (!wallet.walletClient) {
          walletClientConnect();
        } else if (session()) {
          logout(props.api_url).then((r) => setSession(r));
        } else {
          login(props.api_url, wallet.walletClient).then((r) => setSession(r));
        }
      }}
    >
      {!session() ? (
        <img
          src={joinPaths(props.base_url, "gfx/wallet.svg")}
          alt="wallet"
          width="22px"
          class="m-auto"
        />
      ) : (
        <img
          src={joinPaths(props.base_url, "gfx/user.svg")}
          alt="user"
          width="16px"
          class="m-auto"
        />
      )}
      <div class="text-ellipsis text-wallet font-semibold">
        {!wallet.walletClient && !session()
          ? "CONNECT WALLET"
          : !session()
          ? "LOGIN"
          : firstLastChars(session()?.address ?? "", 6, 6)}
      </div>
    </div>
  );
}
