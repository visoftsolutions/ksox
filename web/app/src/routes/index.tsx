import { AssetsProvider } from "~/utils/providers/AssetsProvider";
import { MarketProvider } from "~/utils/providers/MarketProvider";
import Main from "~/components/Main";
import { SessionProvider } from "~/components/Buttons/WalletButton";

export default function App() {
  return (
    <main class="grid h-screen w-screen grid-cols-[72px_240px_1fr_260px_260px] grid-rows-[48px_1fr_280px] gap-[1px] overflow-auto bg-gray-1 font-sanspro text-white">
      <AssetsProvider>
        <MarketProvider>
          <SessionProvider>
            <Main />
          </SessionProvider>
        </MarketProvider>
      </AssetsProvider>
    </main>
  );
}
