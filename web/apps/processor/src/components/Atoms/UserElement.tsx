import { formatDate } from "@packages/utils/formatDate";
import { Address } from "@packages/components/providers/SessionProvider/models";
import { Uuid } from "@packages/types/primitives/uuid";
import firstLastChars from "@packages/utils/firstLastChars";

export interface IUserElement {
  name: string;
  address: Address;
}

export default function UserElement(props: IUserElement) {
  return (
    <div class="rounded-xl cursor-pointer font-sans grid grid-rows-2 items-center gap-2 text-sm font-bold p-2 border border-r-dark-modal-selected">
      <div class="grid grid-cols-[5rem_1fr] items-center justify-items-start overflow-clip">
        <div class="text-r-blue text-ellipsis">name: </div>
        <div class="text-ellipsis">{props.name}</div>
      </div>
      <div class="grid grid-cols-[5rem_1fr] items-center justify-items-start overflow-clip">
        <div class="text-r-blue text-ellipsis">address: </div>
        <div>{firstLastChars(props.address, 8, 8)}</div>
      </div>
    </div>
  );
}
