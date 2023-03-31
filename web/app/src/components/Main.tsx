import Markets from "~/components/Markets";
import CreateChart from "~/components/Chart";
import MainMenu from "~/components/MainMenu";
import CreateOrderBook from "~/components/OrderBook";
import SideMenu from "~/components/SideMenu";
import CreateSubmit from "~/components/Submit";
import CreateTrades from "~/components/Trades";
import { SessionProvider, useSession } from "~/components/Buttons/WalletButton";
import { Dynamic } from "solid-js/web";
import CreateState from "~/components/State";
import { useMarket } from "~/utils/providers/MarketProvider";
import { createEffect } from "solid-js";

export default function Main() {
  const market = useMarket();
  const session = useSession();
  const precision = 3;
  const capacity = 20;

  createEffect(() => {
    console.log(`market: ${market().base_asset?.symbol} - ${market().quote_asset?.symbol}`);
  });

  return (
    <>
      <div class="col-start-2 col-end-6 row-start-1 row-end-2 bg-gray-2">
        <SessionProvider>
          <MainMenu />
        </SessionProvider>
      </div>
      <div class="col-start-1 col-end-2 row-start-1 row-end-4 bg-gray-1">
        <SideMenu />
      </div>
      <div class="col-start-2 col-end-3 row-start-2 row-end-3 bg-gray-2 xl:hidden">
        <Markets />
      </div>
      <div class="col-start-3 col-end-4 row-start-2 row-end-3 bg-gray-1 lg:col-end-5 xl:col-start-2">
        <Dynamic component={CreateChart(market())} />
      </div>
      <div class="col-start-4 col-end-5 row-start-2 row-end-3 bg-gray-2 lg:col-start-5">
        <Dynamic component={CreateOrderBook(market(), precision, capacity)} />
      </div>
      <div class="col-start-5 col-end-6 row-start-2 row-end-3 bg-gray-2 lg:hidden">
        <Dynamic component={CreateTrades(market(), precision, capacity)} />
      </div>
      <div class="col-start-2 col-end-4 row-start-3 row-end-4 bg-gray-2 lg:col-end-5">
        <Dynamic component={CreateState(market(), session() ?? undefined, precision)} />
      </div>
      <div class="col-start-4 col-end-6 row-start-3 row-end-4 bg-gray-2 lg:col-start-5">
        <Dynamic component={CreateSubmit(market(), session() ?? undefined, precision)} />
      </div>
    </>
  );
}
