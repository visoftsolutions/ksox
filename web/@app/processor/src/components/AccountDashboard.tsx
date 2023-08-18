import CurrencyDisplay from "./CurrencyDisplay";
import DepositWithdrawPanel from "./DepositWithdrawPanel";
import { ITransaction } from "./Transactions/Transaction";
import Transactions from "./Transactions/Transactions";

export default function AccountDashboard() {

  const transactionsData: ITransaction[] = [
    {
      title: "OpenAI",
      img: "gfx/bitcoin_placeholder.png",
      date: "17 July",
      hour: "18:12",
      amount: 24.60,
      plus: false,
      currency: "BTC",
    },
    {
      title: "Another Company",
      img: "gfx/ethereum_placeholder.png",
      date: "16 July",
      hour: "09:45",
      amount: 50.00,
      plus: true,
      currency: "ETH",
    },
    {
      title: "Third Company",
      img: "gfx/litecoin_placeholder.png",
      date: "15 July",
      hour: "14:29",
      amount: 10.25,
      plus: false,
      currency: "LTC",
    },
  ];
  
  return (
    <div class="m-6 my-2 rounded-xl bg-r-light-foreground dark:bg-r-dark-foreground">
        <CurrencyDisplay/>
        <DepositWithdrawPanel />
      <div>
        <p class="text-sans mx-5 text-sm text-bold text-r-dark-secondary-text">
          Transactions
        </p>
        <Transactions transactions={transactionsData}/>
      </div>
    </div>
  );
}
