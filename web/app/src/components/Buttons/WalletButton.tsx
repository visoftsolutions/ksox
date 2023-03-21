export interface WalletButtonDisplay {
  onClick?: (e: MouseEvent) => void;
}

export default function WalletButton(props: WalletButtonDisplay) {
  return (
    <div
      class="grid cursor-pointer select-none grid-cols-[auto_auto] grid-rows-[1fr] items-center justify-center gap-4 px-4"
      onClick={(e) => props.onClick?.(e)}
    >
      <div class="text-mainmenu-wallet font-normal">CONNECT WALLET</div>
      <img src="gfx/metamask.webp" class="m-auto w-[22px]" />
    </div>
  );
}