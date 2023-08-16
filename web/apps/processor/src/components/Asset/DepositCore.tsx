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

export function CreateProccessorDeposit(asset?: Asset, precision?: number) {
  return () => (
    <Show when={asset && precision}>
      <DepositCore asset={asset!} precision={precision!} />
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

export function DepositCore(props: { asset: Asset; precision: number }) {
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
            const address_value = address();
            const value = BigInt(
              Math.floor(ev(fmul(props.asset.decimals, amount())))
            );
            if (wallet && address_value && wallet.address) {
              const nonce = (await wallet.publicClient?.readContract({
                address: props.asset.address as Address,
                abi: ERC20_ABI,
                functionName: "nonces",
                account: wallet.address as Address,
                args: [wallet.address as Address],
              })) as bigint;

              const deadline =
                ((await wallet.publicClient?.getBlock())?.timestamp ?? 0n) +
                3600n;

              const domain = {
                name: props.asset.name,
                version: "1",
                chainId: BigInt(wallet.selected_network.network.id),
                verifyingContract: props.asset.address as Address,
              };

              const permit = {
                owner: wallet.address as Address,
                spender: TREASURY_ADDRESS as Address,
                value,
                nonce,
                deadline,
              };

              console.log(domain, permit);

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
                    { name: "name", type: "string" },
                    { name: "version", type: "string" },
                    { name: "chainId", type: "uint256" },
                    { name: "verifyingContract", type: "address" },
                  ],
                },
                primaryType: "Permit",
                message: permit,
              });
              if (signature) {
                const { r, s, v } = splitSig(signature);
                await wallet.walletClient?.writeContract({
                  chain: wallet.selected_network.network,
                  address: TREASURY_ADDRESS,
                  abi: TREASURY_ABI,
                  functionName: "depositPermit",
                  account: wallet.address as Address,
                  args: [
                    props.asset.address as Address,
                    value,
                    deadline,
                    v,
                    r,
                    s,
                  ],
                });
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
