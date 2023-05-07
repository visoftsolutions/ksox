import { Show, onMount } from "solid-js";
import Logo from "~/components/header/Logo";
import MainMenu from "~/components/header/MainMenu";
import Wallet from "./header/Wallet";
import { Nav, useNav } from "~/utils/providers/NavProvider";
import { useCrowdsale } from "~/utils/providers/CrowdsaleProvider";

export default function Header() {
  const nav = useNav();
  const crowdsale = useCrowdsale();

  return (
    <>
      <div class="header-shadow fixed left-0 right-0 top-0 z-10 bg-[#000033]">
        <header class="mx-auto grid max-w-7xl grid-cols-[1fr_auto_1fr] items-center justify-between px-6 py-2">
          <Logo class="col-start-1 col-end-2 justify-self-start" />
          <MainMenu class="col-start-2 col-end-3 justify-self-center" />
          <Show when={nav() == Nav.Token && crowdsale().status == true}>
            <Wallet class="col-start-3 col-end-4 justify-self-end" />
          </Show>
        </header>
      </div>
    </>
  );
}
