import { useAsset } from "~/utils/providers/AssetProvider";
import { useSession } from "~/utils/providers/SessionProvider";
import { usePrecision } from "~/utils/providers/PrecisionProvider";
import CreateAssetInfo from "./Asset/AssetInfo";
import { Dynamic } from "solid-js/web";
import CreateActions from "./Asset/Actions";

export default function Asset() {
  const asset = useAsset();
  const session = useSession();
  const precision = usePrecision();

  return (
    <div class="mx-auto grid max-w-[500px] grid-rows-[auto_1fr] items-center justify-center gap-5 rounded-md bg-gray-2 p-4">
      <div class="row-start-1 row-end-2">
        <Dynamic component={CreateAssetInfo(asset(), session(), precision())} />
      </div>
      <div class="row-start-2 row-end-3 grid ">
        <Dynamic component={CreateActions(asset(), session(), precision())} />
      </div>
    </div>
  );
}
