import NumberInput from "~/components/Inputs/NumberInput";
import { Show, createSignal } from "solid-js";
import { useSession } from "@packages/components/providers/SessionProvider";
import { Asset } from "@packages/types/asset";
import {
  Fraction,
  ev,
  fFromBigint,
  fmul,
} from "@packages/types/primitives/fraction";
import TextInput from "~/components/Inputs/TextInput";
import { api } from "~/root";
import { WithdrawRequest, WithdrawResponse } from "@packages/types/mod";
import { useWallet } from "@packages/components/providers/WalletProvider";
import { Address } from "viem";
import {
  ABI as TREASURY_ABI,
  ADDRESS as TREASURY_ADDRESS,
} from "~/contract/treasury";

export default function CreateWithdraw(asset?: Asset, precision?: number) {
  return () => (
    <Show when={precision && asset}>
      <Withdraw asset={asset!} precision={precision!} />
    </Show>
  );
}

const splitSig = (sig: string) => {
  const pureSig = sig.replace("0x", "");
  const r = "0x" + pureSig.substring(0, 64);
  const s = "0x" + pureSig.substring(64, 128);
  const v = parseInt(pureSig.substring(128, 130), 16);
  return {
    r,
    s,
    v,
  };
};

export function Withdraw(props: { asset: Asset; precision: number }) {
  const [amount, setAmount] = createSignal<Fraction>(fFromBigint(0n));
  const session = useSession();
  const wallet = useWallet();
  const [address, setAddress] = createSignal<Address | undefined>(
    wallet.address,
  );
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
            ${
              session()
                ? "cursor-pointer bg-ksox-2 active:bg-opacity-70"
                : "bg-gray-3"
            }
            select-none items-center justify-center rounded-md  text-markets-label transition-colors duration-75
          `}
          onClick={async () => {
            const address_value = address();
            const value = BigInt(
              Math.floor(ev(fmul(props.asset.decimals, amount()))),
            );
            const deadline = new Date(new Date().getTime() + 60 * 1000);
            if (wallet && address_value && wallet.address) {
              const response = await fetch(`${api}/private/withdraw`, {
                method: "POST",
                headers: {
                  Accept: "application/json",
                  "Content-Type": "application/json",
                },
                credentials: "same-origin",
                body: JSON.stringify(
                  WithdrawRequest.parse({
                    spender: address(),
                    asset: props.asset.address,
                    amount: amount(),
                    deadline,
                  }),
                  (_, v) => (typeof v === "bigint" ? v.toString() : v),
                ),
              })
                .then((r) => r.json())
                .then((r) => WithdrawResponse.parse(r));

              const { r, s, v } = splitSig(response.response);
              await wallet.walletClient?.writeContract({
                chain: wallet.selected_network.network,
                address: TREASURY_ADDRESS,
                abi: TREASURY_ABI,
                functionName: "withdrawPermit",
                account: wallet.address as Address,
                args: [
                  props.asset.address as Address,
                  value,
                  Math.floor(deadline.getTime() / 1000),
                  v,
                  r,
                  s,
                  address(),
                ],
              });
            }
          }}
        >
          Withdraw
        </div>
      </div>
    </>
  );
}
