import Logo from "~/components/Header/Logo";
import MainMenu from "~/components/Header/MainMenu";
import Wallet from "./Header/Wallet";

export default function Header() {
  return (
    <>
      <div class="header-shadow fixed left-0 right-0 top-0 z-20 bg-[#000033]">
        <header class="mx-auto grid max-w-7xl grid-cols-[1fr_auto_1fr] items-center justify-between px-6 py-2">
          <Logo class="col-start-1 col-end-2 justify-self-start" />
          <MainMenu class="col-start-2 col-end-3 justify-self-center" />
          <Wallet class="col-start-3 col-end-4 justify-self-end" />
        </header>
      </div>
    </>
  );
}
