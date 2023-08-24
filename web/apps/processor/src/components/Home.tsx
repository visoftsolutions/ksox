import Header from "~/components/Header";
import AccountDashboard from "~/components/Home/AccountDashboard";

export default function Home() {
  return (
    <div class="grid grid-rows-[128px_1fr] h-full">
      <div class="row-start-1 row-end-2">
        <Header text="Home" />
      </div>
      <div class="row-start-2 row-end-3 relative overflow-clip p-8 bg-r-light-foreground dark:bg-r-dark-foreground rounded-xl">
        <AccountDashboard />
      </div>
    </div>
  );
}
