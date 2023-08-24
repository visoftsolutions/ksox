import { Show, createEffect, createSignal } from "solid-js";
import { useSession } from "@packages/components/providers/SessionProvider";
import { Asset } from "@packages/types/asset";
import {
  Fraction,
  ev,
  fFromBigint,
  fmul,
} from "@packages/types/primitives/fraction";
import { DepositTextInput } from "../Inputs/DepositTextInput";
import { useWallet } from "@packages/components/providers/WalletProvider";
import {
  ABI as TREASURY_ABI,
  ADDRESS as TREASURY_ADDRESS,
} from "../../../../../packages/contracts/treasury";
import { ABI as ERC20_ABI } from "../../../../../packages/contracts/erc20";
import { Address } from "viem";
import { DepositNumberInput } from "../Inputs/DepositNumberInput";
import { WithdrawRequest, WithdrawResponse } from "@packages/types/mod";
import { api } from "~/root";
import { handleWithdraw } from "~/helpers/handleWithdraw";

export function CreateProccessorWithdraw(asset?: Asset, precision?: number) {
  return () => (
    <Show when={asset && precision}>
      <WithdrawCore asset={asset!} precision={precision!} />
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

export function WithdrawCore(props: { asset: Asset; precision: number }) {
  const [amount, setAmount] = createSignal<Fraction>(fFromBigint(0n));
  const session = useSession();
  const wallet = useWallet();
  const [address, setAddress] = createSignal<Address | undefined>(
    wallet.address
  );

  createEffect(() => {
    setAddress(wallet.address);
  });

  createEffect(() => {});

  return (
    <>
      <div class="flex flex-col items-center justify-start gap-3">
        <DepositNumberInput
          precision={props.precision}
          left={"Quantity"}
          right={props.asset.symbol}
          value={amount()}
          onChange={(f) => {
            setAmount(f);
          }}
        />
        <DepositTextInput
          left={"Deposit to"}
          value={address()}
          onChange={(f) => {
            setAddress(f.toLowerCase() as Address);
          }}
        />
        <div
          class={`grid py-3 mt-2 w-full
            ${
              session()
                ? "cursor-pointer bg-ksox-2 active:bg-opacity-70"
                : "bg-gray-3"
            }
            select-none items-center justify-center rounded-lg text-lg transition-colors duration-75
          `}
          onClick={async () => {
            try {
              await handleWithdraw({
                asset: props.asset,
                address_value: address(),
                amount: amount(),
                treasury_address: TREASURY_ADDRESS,
                wallet: wallet,
              });
            } catch (error) {
              console.error("Error during deposit:", error);
            }
          }}
        >
          Withdraw
        </div>
      </div>
    </>
  );
}
