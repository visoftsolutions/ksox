import Header from "~/components/Home/Header";
import AccountDashboard from "~/components/Home/AccountDashboard";

export default function Home() {
  return (
    <div class="grid grid-rows-[128px_1fr] h-full">
      <div class="row-start-1 row-end-2">
        <Header />
      </div>
      <div class="row-start-2 row-end-3 relative overflow-clip">
        <div class="absolute inset-0 overflow-y-auto">
          <AccountDashboard />
        </div>
      </div>
    </div>
  );
}
