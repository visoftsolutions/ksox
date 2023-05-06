import { onMount } from "solid-js";
import Logo from "~/components/header/Logo";
import MainMenu from "~/components/header/MainMenu";

export default function Header() {
  return (
    <>
      <div class="bg-[#000033] z-10 fixed top-0 left-0 right-0 header-shadow">
        <header class="mx-auto px-6 py-2 max-w-7xl md:flex md:flex-row md:justify-between">
          <Logo />
          <MainMenu />
        </header>
      </div>
    </>
    
  );
}
