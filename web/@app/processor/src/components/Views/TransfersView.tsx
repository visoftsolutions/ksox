import HomeHeader from "../Headers/HomeHeader";
import AccountDashboard from "../AccountDashboard";
import TransfersHeader from "../Headers/TransfersHeader";
import Transfers from "../Transfers";

export default function TransfersView() {
    const transfersMockData = [{name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}, {name: "Filip Dziurdzia", text: "You sent 0.0001 BTC", date: "Aug 21", img: "gfx/bitcoin_placeholder.png"}]
  return (
    <div>
      <TransfersHeader />
      <Transfers transfers={transfersMockData}/>
    </div>
  );
}
