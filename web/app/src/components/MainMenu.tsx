import WalletButton from "./Buttons/WalletButton";

export default function MainMenu() {
  return (
    <div class="justify-center, grid h-full grid-cols-[1fr_auto] grid-rows-[1fr] items-center">
      <div class="col-start-2 col-end-3">
        <WalletButton />
      </div>
    </div>
  );
}
