import { setStore, OrderSide } from "~/components/State";

export function init() {
  setStore({
    open_orders: [
      {
        order_time: new Date(),
        asset_pair: ["BTC", "USDT"],
        order_side: OrderSide.Buy,
        price: "2132.13",
        quantity: "22.1133",
        filled_quantity: "22.113",
      },
    ],
    order_history: [
      {
        order_time: new Date(),
        asset_pair: ["BTC", "ETH"],
        order_side: OrderSide.Buy,
        price: "2213.13",
        quantity: "2122.13",
        filled_quantity: "212.13",
      },
    ],
    trade_history: [
      {
        order_time: new Date(),
        asset_pair: ["BTC", "SHIB"],
        order_side: OrderSide.Sell,
        price: "225.13",
        quantity: "22.143",
        filled_quantity: "2132.13",
      },
    ],
  });
}
