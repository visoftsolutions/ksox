import { Show, onMount } from "solid-js";
import Logo from "~/components/header/Logo";
import MainMenu from "~/components/header/MainMenu";
import Wallet from "./header/Wallet";
import { Nav, useNav } from "~/utils/providers/NavProvider";
import { useCrowdsale } from "~/utils/providers/CrowdsaleProvider";

export default function Header() {
  const nav = useNav()
  const crowdsale = useCrowdsale();

  return (
    <>
      <div class="bg-[#000033] z-10 fixed top-0 left-0 right-0 header-shadow">
        <header class="mx-auto px-6 py-2 max-w-7xl grid grid-cols-[1fr_auto_1fr] justify-between items-center">
          <Logo class="col-start-1 col-end-2 justify-self-start" />
          <MainMenu class="col-start-2 col-end-3 justify-self-center" />
          <Show when={nav() == Nav.Token && crowdsale().status == true}>
            <Wallet class="col-start-3 col-end-4 justify-self-end"/>
          </Show>
        </header>
      </div>
    </>
    
  );
}
