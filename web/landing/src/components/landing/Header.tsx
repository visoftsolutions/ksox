import Logo from "./header/Logo";
import MainMenu from "./header/MainMenu";

export default function Header() {
  return (
    <header class="md:flex md:flex-row md:justify-between">
      <Logo />
      <MainMenu />
    </header>
  );
}
