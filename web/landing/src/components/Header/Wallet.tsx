import { DefaultProps } from "~/utils/interfaces";
import NetworkDropdown from "../Token/NetworkDropdown";

export default function Wallet(props: DefaultProps) {
  return (
    <div class={`grid grid-cols-[auto_auto] gap-4 ${props.class}`}>
      <NetworkDropdown />
      <div class="token-linear-wipe-button cursor-pointer rounded-full px-4 py-2 text-center font-lexend font-medium text-text-1">
        Connect
      </div>
    </div>
  );
}
