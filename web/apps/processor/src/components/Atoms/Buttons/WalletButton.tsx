import {
  setSession,
  login,
  useSession,
  logout,
} from "@packages/components/providers/SessionProvider";
import {
  useWallet,
  walletClientConnect,
} from "@packages/components/providers/WalletProvider";
import firstLastChars from "@packages/utils/firstLastChars";
import { createEffect, untrack } from "solid-js";

export default function WalletButton(props: {
  base_url: string;
  api_url: string;
  class?: string;
}) {
  const wallet = useWallet();
  const session = useSession();

  createEffect(() => {
    console.log(wallet.walletClient);
  });

  createEffect(async () => {
    if (wallet.walletClient && untrack(() => !session())) {
      setSession(await login(props.api_url, wallet.walletClient));
    }
  });

  return (
    <button
      class={`cursor-pointer border-r-light-text dark:border-r-dark-text border rounded-xl grid items-center justify-center justify-items-center font-bold text-sm p-2 ${props.class}`}
      onClick={async () => {
        if (!wallet.walletClient) {
          await walletClientConnect();
        } else if (session()) {
          setSession(await logout(props.api_url));
        } else {
          console.log("logging in if");
          setSession(await login(props.api_url, wallet.walletClient));
        }
      }}
    >
      <div class="text-ellipsis">
        {!wallet.walletClient && !session()
          ? "CONNECT"
          : !session()
          ? "LOGIN"
          : firstLastChars(session()?.address ?? "", 6, 6)}
      </div>
    </button>
  );
}
