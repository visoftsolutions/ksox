import { A } from "solid-start";

export default function MainMenu() {
  return (
    <div class="flex items-center justify-between gap-[42px]">
      <A href="/about">
        <span class="text-text-faded hover:text-text-white text-main-menu-button font-medium transition-colors">
          About KSOX
        </span>
      </A>
      <A href="/contact">
        <span class="text-text-faded hover:text-text-white text-main-menu-button font-medium transition-colors">
          Contact
        </span>
      </A>
      <A
        href="/app"
        class="text-text-white hover:bg-text-white bg-primary p-[8px_20px] transition-colors [border-radius:100px] hover:text-primary"
      >
        <span class="text-main-menu-button font-medium">Get Started</span>
      </A>
    </div>
  );
}
