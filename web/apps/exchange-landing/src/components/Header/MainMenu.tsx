import { A } from "solid-start";
import { useCrowdsale } from "~/components/providers/CrowdsaleProvider";

export default function MainMenu(props: { class: string }) {
  const crowdsale = useCrowdsale();
  return (
    <div
      class={`grid grid-flow-col items-center justify-end space-x-6 ${props.class}`}
    >
      <a
        class="main-menu-button rounded-full bg-buttonbg_new px-5 py-2 font-medium text-text-1 hover:bg-text-1 hover:text-buttonbg_new"
        href="https://app.ksox.finance"
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
        class="main-menu-button font-medium text-links_new hover:text-text-1 max-xl:hidden"
        href="/whitepaper/ksox-whitepaper.pdf"
        target="_blank"
      >
        Whitepaper
      </a>
      <a
        class="main-menu-button font-medium text-links_new hover:text-text-1 max-xl:hidden"
        href="/#safety"
      >
        About KSOX
      </a>
      <a
        class="main-menu-button font-medium text-links_new hover:text-text-1 max-xl:hidden"
        href="#contact"
      >
        Contact
      </a>
    </div>
  );
}
