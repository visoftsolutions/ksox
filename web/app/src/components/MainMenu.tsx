import { Uuid } from "~/types/primitives/uuid";
import WalletButton from "./Buttons/WalletButton";

export default function MainMenu(props: { baseAssetId?: Uuid; quoteAssetId?: Uuid }) {
  return (
    <div class="justify-center, grid h-full grid-cols-[1fr_auto] grid-rows-[1fr] items-center">
      <span>{`baseAssetId: ${props.baseAssetId} quoteAssetId: ${props.quoteAssetId}`}</span>
      <div class="col-start-2 col-end-3">
        <WalletButton />
      </div>
    </div>
  );
}
