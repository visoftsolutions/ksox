import { WalletProvider } from "@packages/components/providers/WalletProvider";
import { Asset } from "@packages/types/asset";
import { WithdrawRequest, WithdrawResponse } from "@packages/types/mod";
import { Fraction, ev, fmul } from "@packages/types/primitives/fraction";
import { Address } from "viem";
import { ABI as TREASURY_ABI } from "@packages/contracts/treasury";

interface HandleWithdrawProps {
  address_value: Address;
  asset: Asset;
  amount: Fraction;
  wallet: WalletProvider;
  treasury_address: string;
}

export const handleWithdraw = async ({
  address_value,
  asset,
  amount,
  wallet,
  treasury_address,
}: HandleWithdrawProps) => {
  const value = BigInt(Math.floor(ev(fmul(asset.decimals, amount))));
  const deadline = new Date(new Date().getTime() + 60 * 1000);
  const response = await fetch(`/api/private/withdraw`, {
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    credentials: "same-origin",
    body: JSON.stringify(
      WithdrawRequest.parse({
        spender: address_value,
        asset: asset.address,
        amount: amount,
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
    address: treasury_address as Address,
    abi: TREASURY_ABI,
    functionName: "withdrawPermit",
    account: wallet.address as Address,
    args: [
      asset.address as Address,
      value,
      BigInt(Math.floor(deadline.getTime() / 1000)),
      v,
      r,
      s,
      address_value,
    ],
  });
};

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
