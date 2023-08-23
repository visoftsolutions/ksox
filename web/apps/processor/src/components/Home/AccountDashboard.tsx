import CurrencyDisplay from "~/components/Home/CurrencyDisplay";
import DepositWithdrawPanel from "~/components/Home/DepositWithdrawPanel";
import { ITransaction } from "~/components/Home/TransactionsElement";
import Transactions from "~/components/Home/Transactions";

export default function AccountDashboard() {
  const transactionsData: ITransaction[] = [
    {
      title: "OpenAI",
      img: "gfx/bitcoin_placeholder.png",
      date: "17 July",
      hour: "18:12",
      amount: 24.6,
      plus: false,
      currency: "BTC",
    },
    {
      title: "Another Company",
      img: "gfx/ethereum_placeholder.png",
      date: "16 July",
      hour: "09:45",
      amount: 50.0,
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
    <div class="rounded-xl bg-r-light-foreground dark:bg-r-dark-foreground scrollbar-thumb-r-dark-secondary-text dark:scrollbar-thumb-r-dark-active">
      <CurrencyDisplay />
      <DepositWithdrawPanel />
      <div>
        <p class="text-sans mx-5 text-sm text-bold text-r-dark-secondary-text">
          Transactions
        </p>
        <Transactions transactions={transactionsData} />
      </div>
    </div>
  );
}
