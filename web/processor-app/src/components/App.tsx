import { useMarket } from "~/utils/providers/MarketProvider";
import { useSession } from "~/utils/providers/SessionProvider";
import { usePrecision } from "~/utils/providers/PrecisionProvider";

export default function App() {
  const market = useMarket();
  const session = useSession();
  const precision = usePrecision();
  const capacity = 40;

  return <div></div>;
}
