import Logo from "./Logo";
import MainMenu from "./MainMenu";

export default function Header() {
  return (
    <header class="flex justify-between items-center mt-[24px] mb-[72px]">
      <Logo />
      <MainMenu />
    </header>
  );
}
