import { useSession } from "@web/components/providers/SessionProvider";
import { usePrecision } from "@web/components/providers/PrecisionProvider";
import { useAssets } from "~/components/providers/AssetsProvider";
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
