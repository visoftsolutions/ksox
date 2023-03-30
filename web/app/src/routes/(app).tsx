import Markets from "~/components/Markets";
import Chart from "~/components/Chart";
import MainMenu from "~/components/MainMenu";
import State from "~/components/State";
import OrderBook from "~/components/OrderBook";
import SideMenu from "~/components/SideMenu";
import CreateSubmit from "~/components/Submit";
import Trades from "~/components/Trades";
import { SessionProvider } from "~/components/Buttons/WalletButton";
import { useParams } from "solid-start";
import { Dynamic } from "solid-js/web";



export default function Home() {
  const { baseAssetId, quoteAssetId } = useParams<{ baseAssetId?: string; quoteAssetId?: string }>();

  return (
    <main class="grid h-screen w-screen grid-cols-[72px_240px_1fr_260px_260px] grid-rows-[48px_1fr_280px] gap-[1px] overflow-auto bg-gray-1 font-sanspro text-white">
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
        <Chart />
      </div>
      <div class="col-start-4 col-end-5 row-start-2 row-end-3 bg-gray-2 lg:col-start-5">
        <OrderBook />
      </div>
      <div class="col-start-5 col-end-6 row-start-2 row-end-3 bg-gray-2 lg:hidden">
        <Trades />
      </div>
      <div class="col-start-2 col-end-4 row-start-3 row-end-4 bg-gray-2 lg:col-end-5">
        <SessionProvider>
          <State />
        </SessionProvider>
      </div>
      <div class="col-start-4 col-end-6 row-start-3 row-end-4 bg-gray-2 lg:col-start-5">
        <SessionProvider>
          <Dynamic component={CreateSubmit(baseAssetId, quoteAssetId, 2)} />
        </SessionProvider>
      </div>
    </main>
  );
}
