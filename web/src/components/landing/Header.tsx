import Logo from "./Logo";
import MainMenu from "./MainMenu";

export default function Header() {
  return (
    <header class="mt-[24px] mb-[72px] flex items-center justify-between">
      <Logo />
      <MainMenu />
    </header>
  );
}
