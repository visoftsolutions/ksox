import NumberInput from "../Inputs/NumberInput";
import { Show, createSignal } from "solid-js";
import { api } from "~/root";
import { MintBurnRequest } from "@web/types/mod";
import { useSession } from "@web/components/providers/SessionProvider";
import { Asset } from "@web/types/asset";
import { Fraction, fFromBigint } from "@web/types/primitives/fraction";

export default function CreateMint(asset?: Asset, precision?: number) {
  return () => (
    <Show when={asset && precision}>
      <Mint asset={asset!} precision={precision!} />
    </Show>
  );
}

export function Mint(props: { asset: Asset; precision: number }) {
  const [amount, setAmount] = createSignal<Fraction>(fFromBigint(0n));
  const session = useSession();
  return (
    <>
      <div class="font-lexend text-[32px] font-extralight">Mint assets</div>
      <div class="grid items-center justify-start gap-6">
        <NumberInput
          class="col-start-1 col-end-2 my-4 w-72 bg-gray-1 p-1 text-submit-label"
          precision={props.precision}
          left={"Quantity"}
          right={props.asset.symbol}
          value={amount()}
          onChange={(f) => {
            setAmount(f);
          }}
        />
        <div
          class={`col-start-2 col-end-3 grid h-[32px] w-[100px] 
            ${session() ? "cursor-pointer bg-ksox-2 active:bg-opacity-70" : "bg-gray-3"}
            select-none items-center justify-center rounded-md  text-markets-label transition-colors duration-75
          `}
          onClick={async () => {
            if (session()) {
              await fetch(`${api}/private/mint`, {
                method: "POST",
                headers: {
                  Accept: "application/json",
                  "Content-Type": "application/json",
                },
                credentials: "same-origin",
                body: JSON.stringify(
                  MintBurnRequest.parse({
                    asset_id: props.asset.id,
                    amount: amount(),
                  }),
                  (_, v) => (typeof v === "bigint" ? v.toString() : v)
                ),
              }).then((r) => r.text());
            }
          }}
        >
          Mint
        </div>
      </div>
    </>
  );
}
