import { A } from "solid-start";
import { DefaultProps } from "~/utils/interfaces";

export default function Wallet(props: DefaultProps) {
  return (
    <div class={`flex justify-between ${props.class}`}>network wallet</div>
  );
}
