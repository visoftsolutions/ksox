interface ItemProps {
  title: string;
  description?: string;
}

export default function Item(props: ItemProps) {
  return (
    <div class="grid grid-rows-[auto_auto] gap-1 font-lexend">
      <div class="text-md row-start-1 row-end-2 overflow-hidden text-ellipsis font-normal">{props.title}</div>
      <div class="row-start-2 row-end-3 overflow-hidden text-ellipsis text-sm font-extralight">{props.description}</div>
    </div>
  );
}
