import Benefit from "./Benefit";

export default function Benefits() {
  return (
    <div class="text-benefitsnew grid grid-rows-3 gap-4 md:items-center md:text-text-1  lg:items-center lg:pb-11 ">
      <Benefit>Smart-contracts for keeping users' balances</Benefit>

      <Benefit>
        Modern web app design, most efficient and scalable solutions
      </Benefit>

      <Benefit>Cairo language for proving computation integrity</Benefit>
    </div>
  );
}
