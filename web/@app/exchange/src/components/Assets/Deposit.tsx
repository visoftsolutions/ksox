import NumberInput from "../Inputs/NumberInput";
import { Show, createSignal } from "solid-js";
import { useSession } from "@web/components/providers/SessionProvider";
import { Asset } from "@web/types/asset";
import { Fraction, fFromBigint } from "@web/types/primitives/fraction";
import TextInput from "../Inputs/TextInput";

export default function CreateDeposit(asset?: Asset, precision?: number) {
  return () => (
    <Show when={asset && precision}>
      <Deposit asset={asset!} precision={precision!} />
    </Show>
  );
}

export function Deposit(props: { asset: Asset; precision: number }) {
  const [amount, setAmount] = createSignal<Fraction>(fFromBigint(0n));
  const [address, setAddress] = createSignal<string>("");
  const session = useSession();
  return (
    <>
      <div class="font-lexend text-[32px] font-extralight">Deposit assets</div>
      <div class="grid items-center justify-start gap-2">
        <NumberInput
          class="col-start-1 col-end-3 mt-4 w-72 bg-gray-1 p-1 text-submit-label"
          precision={props.precision}
          left={"Quantity"}
          right={props.asset.symbol}
          value={amount()}
          onChange={(f) => {
            setAmount(f);
          }}
        />
        <TextInput
          class="col-start-1 col-end-3 row-start-2 w-72 bg-gray-1 p-1 text-submit-label"
          precision={props.precision}
          left={"Deposit to"}
          value={address()}
          onChange={(f) => {
            setAddress(f);
          }}
        />
        <div
          class={`col-start-1 col-end-2 row-start-3 grid h-[32px] w-[100px] 
            ${session() ? "cursor-pointer bg-ksox-2 active:bg-opacity-70" : "bg-gray-3"}
            select-none items-center justify-center rounded-md  text-markets-label transition-colors duration-75
          `}
        >
          Deposit
        </div>
      </div>
    </>
  );
}