import { DefaultProps } from "~/utils/interfaces";
import { useCrowdsale } from "~/utils/providers/CrowdsaleProvider";

export default function MainMenu(props: DefaultProps) {
  const crowdsale = useCrowdsale();
  return (
    <div class={`flex items-center justify-end space-x-6 max-[850px]:hidden ${props.class}`}>
      <a
        class="main-menu-button rounded-full bg-buttonbg_new px-5 py-2 font-medium text-text-1 hover:bg-text-1 hover:text-buttonbg_new"
        href="/app"
      >
        Launch App
      </a>
      <a
        class={`main-menu-button font-extrabold ${crowdsale().status ? "token-linear-wipe-text" : "text-gray-700"}`}
        href="/token"
      >
        Buy Token
      </a>
      <a
        class="main-menu-button font-medium text-links_new hover:text-text-1"
        href="/whitepaper/ksox-whitepaper.pdf"
        target="_blank"
      >
        Whitepaper
      </a>
      <a
        class="main-menu-button font-medium text-links_new hover:text-text-1"
        href="/#section-1"
      >
        About KSOX
      </a>
      <a
        class="main-menu-button font-medium text-links_new hover:text-text-1"
        href="#section-4"
      >
        Contact
      </a>
      
    </div>
  );
}
