import { AssetInfo } from "../Assets";
import NumberInput from "../Inputs/NumberInput";
import { createSignal } from "solid-js";
import { fromWei, toWei } from "~/utils/converters/wei";
import { format, parse } from "numerable";
import { formatTemplate } from "~/utils/precision";
import { api } from "~/root";
import { MintBurnRequest } from "~/types/mod";

export default function Mint(props: { asset: AssetInfo; precision: number }) {
  const [amount, setAmount] = createSignal<bigint>(0n);
  return (
    <>
      <div class="font-lexend text-[32px] font-extralight">Mint assets</div>
      <div class="grid items-center justify-start gap-6">
        <NumberInput
          class="col-start-1 col-end-2 my-4 w-72"
          precision={props.precision}
          left={"Quantity"}
          right={props.asset.symbol}
          value={format(fromWei(amount()), formatTemplate(props.precision))}
          onChange={(e) => {
            const value = toWei(parse((e.target as HTMLInputElement).value ?? 0) ?? 0);
            setAmount(value);
          }}
        />
        <div
          class="col-start-2 col-end-3 grid h-[32px] w-[100px] cursor-pointer select-none items-center justify-center rounded-md bg-ksox-2 text-markets-label transition-colors duration-75 active:bg-opacity-70"
          onClick={async () => {
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
            })
              .then((r) => r.text())
              .then((r) => console.log(r));
          }}
        >
          Mint
        </div>
      </div>
    </>
  );
}
