import { WalletProvider } from "@packages/components/providers/WalletProvider";
import { Asset } from "@packages/types/asset";
import { Fraction, ev, fmul } from "@packages/types/primitives/fraction";
import { Address } from "viem";
import { ABI as ERC20_ABI } from "@packages/contracts/erc20";
import { ABI as TREASURY_ABI } from "@packages/contracts/treasury";

interface HandleDepositProps {
  asset: Asset;
  amount: Fraction;
  wallet: WalletProvider;
  treasury_address: string;
}

export const handleDeposit = async ({
  asset,
  amount,
  wallet,
  treasury_address,
}: HandleDepositProps) => {
  const value = BigInt(Math.floor(ev(fmul(asset.decimals, amount))));
  const nonce = (await wallet.publicClient?.readContract({
    address: asset.address as Address,
    abi: ERC20_ABI,
    functionName: "nonces",
    account: wallet.address as Address,
    args: [wallet.address as Address],
  })) as bigint;

  const deadline = BigInt(
    Math.floor((new Date().getTime() + 60 * 1000) / 1000)
  );

  const domain = {
    name: asset.name,
    version: "1",
    chainId: BigInt(wallet.selected_network.network.id),
    verifyingContract: asset.address as Address,
  };

  const permit = {
    owner: wallet.address as Address,
    spender: treasury_address as Address,
    value,
    nonce,
    deadline,
  };

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
      address: treasury_address as Address,
      abi: TREASURY_ABI,
      functionName: "depositPermit",
      account: wallet.address as Address,
      args: [asset.address as Address, value, deadline, v, r, s],
    });
  }
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
