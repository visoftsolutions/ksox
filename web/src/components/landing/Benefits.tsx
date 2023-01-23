import Benefit from "./Benefit";

export default function Benefits() {
  return (
    <div class="flex flex-col items-start gap-[4px] text-hero-benefit text-text-white">
      <Benefit>Smart-contracts for keeping users' balances</Benefit>

      <Benefit>
        Modern web app design, most efficient and scalable solutions
      </Benefit>

      <Benefit>Cairo language for proving computation integrity</Benefit>
    </div>
  );
}
