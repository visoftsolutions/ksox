import NumberInput from "../Inputs/NumberInput";
import { Show, createSignal } from "solid-js";
import { fToWeiCeil } from "~/utils/converters/wei";
import { api } from "~/root";
import { MintBurnRequest } from "~/types/mod";
import { useSession } from "~/utils/providers/SessionProvider";
import { Asset } from "~/types/asset";
import { Fraction, fFromBigint } from "~/types/primitives/fraction";

export default function CreateBurn(asset?: Asset, precision?: number) {
  return () => (
    <Show when={asset && precision}>
      <Burn asset={asset!} precision={precision!} />
    </Show>
  );
}

export function Burn(props: { asset: Asset; precision: number }) {
  const [amount, setAmount] = createSignal<Fraction>(fFromBigint(0n));
  const session = useSession();

  return (
    <>
      <div class="font-lexend text-[32px] font-extralight">Burn assets</div>
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
              await fetch(`${api}/private/burn`, {
                method: "POST",
                headers: {
                  Accept: "application/json",
                  "Content-Type": "application/json",
                },
                credentials: "same-origin",
                body: JSON.stringify(
                  MintBurnRequest.parse({
                    asset_id: props.asset.id,
                    amount: fToWeiCeil(amount()),
                  }),
                  (_, v) => (typeof v === "bigint" ? v.toString() : v)
                ),
              }).then((r) => r.text());
            }
          }}
        >
          Burn
        </div>
      </div>
    </>
  );
}
