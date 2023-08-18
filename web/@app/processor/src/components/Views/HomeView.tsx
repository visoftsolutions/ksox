import HomeHeader from "../Headers/HomeHeader";
import AccountDashboard from "../AccountDashboard";
import SearchBar from "../Atoms/SearchBar";

export default function HomeView() {
  return (
    <div>
      <p class="hidden xl:block text-3xl font-sans m-6 font-bold">Home</p>
      <div class="hidden xl:block m-6">
        <SearchBar/>
      </div>
      <div class="xl:hidden">
        <HomeHeader />
      </div>
      <AccountDashboard />
    </div>
  );
}
