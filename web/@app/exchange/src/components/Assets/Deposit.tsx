import NumberInput from "../Inputs/NumberInput";
import { Show, createEffect, createSignal } from "solid-js";
import { useSession } from "@web/components/providers/SessionProvider";
import { Asset } from "@web/types/asset";
import { Fraction, ev, fFromBigint, fmul } from "@web/types/primitives/fraction";
import TextInput from "../Inputs/TextInput";
import { useWallet } from "@web/components/providers/WalletProvider";
import { ABI as TREASURY_ABI, ADDRESS as TREASURY_ADDRESS } from "~/contract/treasury";
import { ABI as ERC20_ABI } from "~/contract/erc20";
import { Address } from "viem";
import { sepolia } from "@wagmi/core";

export default function CreateDeposit(asset?: Asset, precision?: number) {
  return () => (
    <Show when={asset && precision}>
      <Deposit asset={asset!} precision={precision!} />
    </Show>
  );
}

const splitSig = (sig: string) => {
  const pureSig = sig.replace("0x", "")
  const r = "0x"+pureSig.substring(0, 64)
  const s = "0x"+pureSig.substring(64, 128)
  const v = parseInt(pureSig.substring(128, 130), 16);
  return {
    r, s, v
  }
}

export function Deposit(props: { asset: Asset; precision: number }) {
  const [amount, setAmount] = createSignal<Fraction>(fFromBigint(0n));
  const session = useSession();
  const wallet = useWallet();
  const [address, setAddress] = createSignal<Address | undefined>(wallet.address);

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
            ${session() ? "cursor-pointer bg-ksox-2 active:bg-opacity-70" : "bg-gray-3"}
            select-none items-center justify-center rounded-md  text-markets-label transition-colors duration-75
          `}
          onClick={async () => {
            const address_value = address();
            const value = BigInt(Math.floor(ev(fmul(props.asset.decimals, amount()))));
            console.log(wallet, address_value, wallet.address)
            if (wallet && address_value && wallet.address) {
              
              let nonce = await wallet.publicWSClient?.readContract({
                address: props.asset.address as Address,
                abi: ERC20_ABI,
                functionName: 'nonces',
                account: wallet.address as Address,
                args: [wallet.address as Address]
              }) as bigint

              let deadline = ((await wallet.publicWSClient?.getBlock())?.timestamp ?? 0n) + 3600n;

              const domain = { 
                name: "TokenPermit",
                version: "1",
                chainId: 11155111n,
                verifyingContract: props.asset.address as Address,
              }

              const permit = { 
                owner: wallet.address as Address, 
                spender: TREASURY_ADDRESS as Address, 
                value, 
                nonce,
                deadline
              }

              const signature = await wallet.walletClient?.signTypedData({
                account: wallet.address as Address,
                domain,
                types: {
                  Permit: [
                    { name: "owner", type: "address" },
                    { name: "spender", type: "address" },
                    { name: "value", type: "uint256" },
                    { name: "nonce", type: "uint256" },
                    { name: "deadline", type: "uint256" },
                  ],
                  EIP712Domain: [
                    { name: 'name', type: 'string' },
                    { name: 'version', type: 'string' },
                    { name: 'chainId', type: 'uint256' },
                    { name: 'verifyingContract', type: 'address' },
                  ],
                },
                primaryType: 'Permit',
                message: permit,
              });
              if (signature) {
                const {r, s, v} = splitSig(signature);
                await wallet.walletClient?.writeContract({
                  address: TREASURY_ADDRESS,
                  abi: TREASURY_ABI,
                  functionName: 'depositPermit',
                  account: wallet.address as Address,
                  args: [
                    props.asset.address as Address,
                    value,
                    deadline,
                    v,r,s
                  ]
                })
              }
            }
          }}
        >
          Deposit
        </div>
      </div>
    </>
  );
}