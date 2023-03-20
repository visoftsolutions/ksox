import Assets from "~/components/Assets";
import Chart from "~/components/Chart";
import MainMenu from "~/components/MainMenu";
import OpenOrders from "~/components/OpenOrders";
import Orderbook from "~/components/OrderBook";
import SideMenu from "~/components/SideMenu";
import Submit from "~/components/Submit";

export default function Home() {
  return (
    <main class="grid h-screen w-screen grid-cols-[72px_240px_1fr_260px_260px] grid-rows-[48px_1fr_280px] gap-[1px] overflow-auto bg-gray-1 font-sanspro text-white">
      <div class="col-start-2 col-end-6 row-start-1 row-end-2 bg-gray-2">
        <MainMenu />
      </div>
      <div class="col-start-1 col-end-2 row-start-1 row-end-4 bg-gray-1">
        <SideMenu />
      </div>
      <div class="col-start-2 col-end-3 row-start-2 row-end-3 bg-gray-2 xl-max:hidden">
        <Assets />
      </div>
      <div class="col-start-3 col-end-4 row-start-2 row-end-3 bg-gray-1 lg-max:col-end-5 xl-max:col-start-2">
        <Chart />
      </div>
      <div class="col-start-4 col-end-6 row-start-2 row-end-3 bg-gray-2 lg-max:col-start-5">
        <Orderbook />
      </div>
      <div class="col-start-2 col-end-4 row-start-3 row-end-4 bg-gray-2 lg-max:col-end-5">
        <OpenOrders />
      </div>
      <div class="col-start-4 col-end-6 row-start-3 row-end-4 bg-gray-2 lg-max:col-start-5">
        <Submit />
      </div>
    </main>
  );
}
