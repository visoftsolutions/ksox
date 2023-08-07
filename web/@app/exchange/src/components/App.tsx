import Markets from "~/components/App/Markets";
import CreateChart from "~/components/App/Chart";
import CreateOrderBook from "~/components/App/OrderBook";
import CreateSubmit from "~/components/App/Submit";
import CreateTrades from "~/components/App/Trades";
import { Dynamic } from "solid-js/web";
import CreateState from "~/components/App/State";
import { useMarket } from "~/components/providers/MarketProvider";
import { useSession } from "@web/components/providers/SessionProvider";
import { usePrecision } from "@web/components/providers/PrecisionProvider";

export default function App() {
  const market = useMarket();
  const session = useSession();
  const precision = usePrecision();
  const capacity = 40;

  return (
    <>
      <div class="col-start-2 col-end-3 row-start-2 row-end-3 bg-gray-2 xl:hidden">
        <Markets />
      </div>
      <div class="relative col-start-3 col-end-4 row-start-2 row-end-3 bg-gray-1 lg:col-end-5 xl:col-start-2">
        <Dynamic component={CreateChart(market())} />
      </div>
      <div class="col-start-4 col-end-5 row-start-2 row-end-3 bg-gray-2 lg:col-start-5">
        <Dynamic component={CreateOrderBook(market(), precision(), capacity)} />
      </div>
      <div class="col-start-5 col-end-6 row-start-2 row-end-3 bg-gray-2 lg:hidden">
        <Dynamic component={CreateTrades(market(), precision(), capacity)} />
      </div>
      <div class="col-start-2 col-end-4 row-start-3 row-end-4 bg-gray-2 lg:col-end-5">
        <Dynamic
          component={CreateState(market(), session(), precision(), capacity)}
        />
      </div>
      <div class="col-start-4 col-end-6 row-start-3 row-end-4 bg-gray-2 lg:col-start-5">
        <Dynamic component={CreateSubmit(market(), session(), precision())} />
      </div>
    </>
  );
}
