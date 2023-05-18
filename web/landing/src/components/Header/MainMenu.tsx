import { A } from "solid-start";
import { DefaultProps } from "~/utils/interfaces";
import { useCrowdsale } from "~/utils/providers/CrowdsaleProvider";

export default function MainMenu(props: DefaultProps) {
  const crowdsale = useCrowdsale();
  return (
    <div
      class={`flex items-center justify-end space-x-6 max-[850px]:hidden ${props.class}`}
    >
      <a
        class="main-menu-button rounded-full bg-buttonbg_new px-5 py-2 font-medium text-text-1 hover:bg-text-1 hover:text-buttonbg_new"
        href="/app"
      >
        Launch App
      </a>
      <A
        class={`main-menu-button font-extrabold ${
          crowdsale.phaseContract.isPhaseActive
            ? "token-linear-wipe-text"
            : "text-gray-700"
        }`}
        href="/token"
      >
        Buy Token
      </A>
      <a
        class="main-menu-button font-medium text-links_new hover:text-text-1"
        href="/whitepaper/ksox-whitepaper.pdf"
        target="_blank"
      >
        Whitepaper
      </a>
      <a
        class="main-menu-button font-medium text-links_new hover:text-text-1"
        href="/#safety"
      >
        About KSOX
      </a>
      <a
        class="main-menu-button font-medium text-links_new hover:text-text-1"
        href="#contact"
      >
        Contact
      </a>
    </div>
  );
}
