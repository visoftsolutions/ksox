import HomeHeader from "../Headers/HomeHeader";
import AccountDashboard from "../AccountDashboard";
import TransfersHeader from "../Headers/TransfersHeader";
import Transfers from "../Transfers";
import NewButton from "../Atoms/Buttons/NewButton";
import SearchBar from "../Atoms/SearchBar";

export default function TransfersView() {
    const transfersMockData = [{name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}]
  return (
    <div>
      <div class="flex flex-row items-center justify-between m-6">
      <p class="hidden xl:block text-3xl font-sans font-bold">Transfers</p>
      <NewButton onClick={() => {}}/>
      </div>
      <div class="hidden xl:block m-6">
        <SearchBar/>
      </div>
      <div class="xl:hidden">
      <TransfersHeader />
      </div>
      <Transfers transfers={transfersMockData}/>
    </div>
  );
}
