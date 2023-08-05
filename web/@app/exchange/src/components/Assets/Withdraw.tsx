import NumberInput from "../Inputs/NumberInput";
import { Show, createSignal } from "solid-js";
import { useSession } from "@web/components/providers/SessionProvider";
import { Asset } from "@web/types/asset";
import { Fraction, fFromBigint } from "@web/types/primitives/fraction";
import TextInput from "../Inputs/TextInput";
import { api } from "~/root";
import { WithdrawRequest } from "@web/types/mod";
import { useWallet } from "@web/components/providers/WalletProvider";
import { Address } from "viem";

export default function CreateWithdraw(asset?: Asset, precision?: number) {
  return () => (
    <Show when={precision && asset}>
      <Withdraw asset={asset!} precision={precision!} />
    </Show>
  );
}

export function Withdraw(props: { asset: Asset; precision: number }) {
  const [amount, setAmount] = createSignal<Fraction>(fFromBigint(0n));
  const session = useSession();
  const wallet = useWallet();
  const [address, setAddress] = createSignal<Address | undefined>(wallet.address);
  return (
    <>
      <div class="font-lexend text-[32px] font-extralight">Withdraw assets</div>
      <div class="grid items-center justify-start gap-3">
        <NumberInput
          class="w-96 bg-gray-1 p-1 text-submit-label"
          precision={props.precision}
          left={"Quantity"}
          right={props.asset.symbol}
          value={amount()}
          onChange={(f) => {
            setAmount(f);
          }}
        />
        <TextInput
          class="w-96 bg-gray-1 p-1 text-submit-label"
          precision={props.precision}
          left={"Withdraw to"}
          value={address()}
          onChange={(f) => {
            setAddress(f.toLowerCase() as Address);
          }}
        />
        <div
          class={`grid h-[32px] w-[100px] 
            ${session() ? "cursor-pointer bg-ksox-2 active:bg-opacity-70" : "bg-gray-3"}
            select-none items-center justify-center rounded-md  text-markets-label transition-colors duration-75
          `}
          onClick={async () => {
            if (session()) {
              await fetch(`${api}/private/withdraw`, {
                method: "POST",
                headers: {
                  Accept: "application/json",
                  "Content-Type": "application/json",
                },
                credentials: "same-origin",
                body: JSON.stringify(
                  WithdrawRequest.parse({
                    maker_address: session()?.address,
                    taker_address: address(),
                    asset_address: props.asset.address,
                    amount: amount(),
                  }),
                  (_, v) => (typeof v === "bigint" ? v.toString() : v)
                ),
              }).then((r) => r.text());
            }
          }}
        >
          Withdraw
        </div>
      </div>
    </>
  );
}