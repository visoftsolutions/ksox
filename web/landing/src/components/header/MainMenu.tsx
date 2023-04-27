export default function MainMenu() {
  return (
    <div class="flex items-center justify-end space-x-6 max-[850px]:hidden">
      <a
        class="main-menu-button font-medium text-links_new hover:text-text-1"
        href="#section-1"
      >
        About KSOX
      </a>
      <a
        class="main-menu-button font-medium text-links_new hover:text-text-1"
        href="#section-4"
      >
        Contact
      </a>
      <a
        class="main-menu-button rounded-full bg-buttonbg_new px-5 py-2 font-medium text-text-1 hover:bg-text-1 hover:text-buttonbg_new "
        href="#section-1"
      >
        Get Started
      </a>
    </div>
  );
}
