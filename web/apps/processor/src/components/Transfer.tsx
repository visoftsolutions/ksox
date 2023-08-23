import Transfers from "~/components/Transfer/Transfers";
import Header from "~/components/Transfer/Header";

export default function Transfer() {
  const transfersMockData = [
    {
      name: "Filip Dziurdzia",
      text: "You sent 0.0001 BTC",
      date: new Date(Date.now()),
      img: "gfx/bitcoin_placeholder.png",
    },
    {
      name: "Filip Dziurdzia",
      text: "You sent 0.0001 BTC",
      date: new Date(Date.now()),
      img: "gfx/bitcoin_placeholder.png",
    },
    {
      name: "Filip Dziurdzia",
      text: "You sent 0.0001 BTC",
      date: new Date(Date.now()),
      img: "gfx/bitcoin_placeholder.png",
    },
    {
      name: "Filip Dziurdzia",
      text: "You sent 0.0001 BTC",
      date: new Date(Date.now()),
      img: "gfx/bitcoin_placeholder.png",
    },
    {
      name: "Filip Dziurdzia",
      text: "You sent 0.0001 BTC",
      date: new Date(Date.now()),
      img: "gfx/bitcoin_placeholder.png",
    },
  ];
  return (
    <div class="grid grid-rows-[128px_1fr] p-6 h-full">
      <div class="row-start-1 row-end-2">
        <Header />
      </div>
      <div class="row-start-2 row-end-3 relative overflow-clip">
        <div class="absolute inset-0 overflow-y-auto">
          <Transfers transfers={transfersMockData} />
        </div>
      </div>
    </div>
  );
}
