import Markets, { useMarket } from "~/components/Markets";
import CreateChart from "~/components/Chart";
import MainMenu from "~/components/MainMenu";
import State from "~/components/State";
import CreateOrderBook from "~/components/OrderBook";
import SideMenu from "~/components/SideMenu";
import CreateSubmit from "~/components/Submit";
import CreateTrades from "~/components/Trades";
import { SessionProvider, useSession } from "~/components/Buttons/WalletButton";
import { useParams } from "solid-start";
import { Uuid } from "~/types/primitives/uuid";
import { Dynamic } from "solid-js/web";
import CreateState from "~/components/State";

export default function App() {
  const params = useParams<{ baseAssetId?: Uuid; quoteAssetId?: Uuid }>();
  const market = useMarket();
  const session = useSession();
  const precission = 3;
  const capacity = 20;

  return (
    <main class="grid h-screen w-screen grid-cols-[72px_240px_1fr_260px_260px] grid-rows-[48px_1fr_280px] gap-[1px] overflow-auto bg-gray-1 font-sanspro text-white">
      <div class="col-start-2 col-end-6 row-start-1 row-end-2 bg-gray-2">
        <SessionProvider>
          <MainMenu baseAssetId={params.baseAssetId} quoteAssetId={params.quoteAssetId} />
        </SessionProvider>
      </div>
      <div class="col-start-1 col-end-2 row-start-1 row-end-4 bg-gray-1">
        <SideMenu />
      </div>
      <div class="col-start-2 col-end-3 row-start-2 row-end-3 bg-gray-2 xl:hidden">
        <Markets />
      </div>
      <div class="col-start-3 col-end-4 row-start-2 row-end-3 bg-gray-1 lg:col-end-5 xl:col-start-2">
        <Dynamic component={CreateChart(market()?.quote_asset, market()?.base_asset)} />
      </div>
      <div class="col-start-4 col-end-5 row-start-2 row-end-3 bg-gray-2 lg:col-start-5">
        <Dynamic component={CreateOrderBook(market()?.quote_asset, market()?.base_asset, precission, capacity)} />
      </div>
      <div class="col-start-5 col-end-6 row-start-2 row-end-3 bg-gray-2 lg:hidden">
        <Dynamic component={CreateTrades(market()?.quote_asset, market()?.base_asset, precission)} />
      </div>
      <div class="col-start-2 col-end-4 row-start-3 row-end-4 bg-gray-2 lg:col-end-5">
        <Dynamic component={CreateState(market()?.quote_asset, market()?.base_asset, session() ?? undefined)} />
      </div>
      <div class="col-start-4 col-end-6 row-start-3 row-end-4 bg-gray-2 lg:col-start-5">
        <Dynamic component={CreateSubmit(market()?.quote_asset, market()?.base_asset, session() ?? undefined, precission)} />
      </div>
    </main>
  );
}
