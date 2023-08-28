import { WalletProvider } from "@packages/components/providers/WalletProvider";
import { Asset } from "@packages/types/asset";
import { Fraction, fToBigint, fmul } from "@packages/types/primitives/fraction";
import { Address } from "viem";
import { ABI as ERC20_ABI } from "@packages/contracts/erc20";
import { ABI as TREASURY_ABI } from "@packages/contracts/treasury";

interface HandleDepositProps {
  asset: Asset;
  amount: Fraction;
  wallet: WalletProvider;
  treasuryAddress: string;
}

export const handleDeposit = async ({
  asset,
  amount,
  wallet,
  treasuryAddress,
}: HandleDepositProps) => {
  const value = fToBigint(fmul(asset.decimals, amount));
  const response = await wallet.walletClient?.writeContract({
    chain: wallet.selected_network.network,
    address: asset.address as Address,
    abi: ERC20_ABI,
    functionName: "approve",
    account: wallet.address as Address,
    args: [treasuryAddress as Address, value],
  });

  if (response) {
    console.log(
      await wallet.publicClient?.waitForTransactionReceipt({ hash: response }),
    );
    await wallet.walletClient?.writeContract({
      chain: wallet.selected_network.network,
      address: treasuryAddress as Address,
      abi: TREASURY_ABI,
      functionName: "deposit",
      account: wallet.address as Address,
      args: [asset.address as Address, value],
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
