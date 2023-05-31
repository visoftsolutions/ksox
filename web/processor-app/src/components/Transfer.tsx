import { createMemo } from "solid-js";
import { useAssets } from "~/utils/providers/AssetsProvider";
import { usePrecision } from "~/utils/providers/PrecisionProvider";
import { useSession } from "~/utils/providers/SessionProvider";

enum Tab {
  Mint,
  Burn,
  History,
  OwnTransfer,
}

export default function Transfer() {
  const assets = useAssets();
  const session = useSession();
  const precision = usePrecision();
  const assetsList = createMemo(() => [...assets().values()]);

  return <div></div>;
}
