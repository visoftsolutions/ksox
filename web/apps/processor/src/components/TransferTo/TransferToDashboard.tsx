import { useWallet } from "@packages/components/providers/WalletProvider";
import CurrencyDisplay from "~/components/Home/CurrencyDisplay";
import { useSelectedAsset } from "../providers/SelectedAssetProvider";
import firstLastChars from "@packages/utils/firstLastChars";
import { api, base } from "~/root";
import { joinPaths } from "solid-start/islands/server-router";
import { useParams } from "@solidjs/router";
import NumberInput from "../Inputs/NumberInput";
import ActionButton from "../Atoms/Buttons/ActionButton";
import { usePrecision } from "@packages/components/providers/PrecisionProvider";
import { createSignal } from "solid-js";
import { Fraction } from "@packages/types/primitives/fraction";
import { TransferRequest } from "@packages/types/mod";
import { useSelectedUser } from "../providers/SelectedUserProvider";
import { useSession } from "@packages/components/providers/SessionProvider";
import { Dynamic } from "solid-js/web";
import { CreateTransfers } from "../Home/AccountDashboard";

export default function TransferToDashboard(props: { search?: string }) {
  const wallet = useWallet();
  const params = useParams();
  const selectedUser = useSelectedUser();
  const session = useSession();
  const precision = usePrecision();
  const { selectedAsset } = useSelectedAsset();
  const [amount, setAmount] = createSignal(
    Fraction.parse({ numer: 0, denom: 1 }),
  );

  return (
    <div class="grid grid-rows-[auto_auto_auto_1fr] h-full gap-6">
      <CurrencyDisplay />
      <div class="font-sans font-bold grid xl:grid-cols-[auto_auto_auto] grid-rows-[auto_auto_auto] justify-items-center items-center gap-1 justify-self-center">
        <div>{firstLastChars(wallet.address?.toLowerCase() ?? "", 8, 8)}</div>
        <img
          class="xl:rotate-0 rotate-90"
          src={joinPaths(base, "/gfx/right_arrow.svg")}
          width={24}
          height={24}
        />
        <div>
          {selectedUser.selectedUser()?.name ??
            firstLastChars(params.address?.toLowerCase() ?? "", 8, 8) ??
            ""}
        </div>
      </div>
      <div class="grid grid-cols-[auto_auto] gap-5">
        <NumberInput
          class="w-full p-1 border text-md justify-self-center"
          precision={3}
          left={"Quantity"}
          right={selectedAsset()?.symbol}
          value={amount()}
          onChange={(f) => {
            setAmount(f);
          }}
        />
        <ActionButton
          text="Transfer"
          onClick={async () => {
            const asset = selectedAsset();
            const to_user = params.address;
            if (asset && to_user) {
              console.log(amount());
              await fetch(`${api}/private/transfer`, {
                method: "POST",
                headers: {
                  Accept: "application/json",
                  "Content-Type": "application/json",
                },
                credentials: "same-origin",
                body: JSON.stringify(
                  TransferRequest.parse({
                    from_user_address: session()?.address,
                    to_user_address: to_user,
                    asset_id: selectedAsset()?.id,
                    amount: amount(),
                  }),
                  (_, v) => (typeof v === "bigint" ? v.toString() : v),
                ),
              }).then((r) => r.text());
            }
          }}
        />
      </div>
      <div class="relative">
        <div class="absolute inset-0 overflow-y-auto">
          <Dynamic component={CreateTransfers(session())} />
        </div>
      </div>
    </div>
  );
}
