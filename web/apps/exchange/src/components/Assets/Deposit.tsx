import NumberInput from "../Inputs/NumberInput";
import { Show, createEffect, createSignal } from "solid-js";
import { useSession } from "@packages/components/providers/SessionProvider";
import { Asset } from "@packages/types/asset";
import { Fraction, fFromBigint } from "@packages/types/primitives/fraction";
import TextInput from "../Inputs/TextInput";
import { useWallet } from "@packages/components/providers/WalletProvider";
import { Address } from "viem";
import { useContractAddress } from "@packages/components/providers/ContractAddressProvider";
import { handleDeposit } from "@packages/utils/handlers/depositPermit";

export default function CreateDeposit(asset?: Asset, precision?: number) {
  return () => (
    <Show when={asset && precision}>
      <Deposit asset={asset!} precision={precision!} />
    </Show>
  );
}

export function Deposit(props: { asset: Asset; precision: number }) {
  const contractAddress = useContractAddress();
  const [amount, setAmount] = createSignal<Fraction>(fFromBigint(0n));
  const session = useSession();
  const wallet = useWallet();
  const [address, setAddress] = createSignal<Address | undefined>(
    wallet.address,
  );

  createEffect(() => {
    setAddress(wallet.address);
  });

  return (
    <>
      <div class="font-lexend text-[32px] font-extralight">Deposit assets</div>
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
          left={"Deposit to"}
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
            const asset = props.asset;
            const treasury = contractAddress!();
            if (asset && treasury) {
              await handleDeposit({
                asset: props.asset,
                amount: amount(),
                wallet,
                treasuryAddress: treasury,
              });
            }
          }}
        >
          Deposit
        </div>
      </div>
    </>
  );
}
