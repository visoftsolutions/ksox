import { useSession } from "~/utils/providers/SessionProvider";
import { usePrecision } from "~/utils/providers/PrecisionProvider";
import { useAssets } from "~/utils/providers/AssetsProvider";
import { Dynamic } from "solid-js/web";
import CreateWealth from "./App/Wealth";

export default function App() {
  const assets = useAssets();
  const session = useSession();
  const precision = usePrecision();

  return (
    <div class="mx-auto grid max-w-[500px] grid-cols-[1fr] items-center justify-center gap-5 rounded-md bg-gray-2 p-4">
      <Dynamic component={CreateWealth(assets(), session(), precision())} />
    </div>
  );
}
