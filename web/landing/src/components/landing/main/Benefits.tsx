import Spacing from "../../Spacing";
import Benefit from "./Benefit";

export default function Benefits() {
  return (
    <div class="flex flex-col items-center text-hero-benefit">
      <Benefit class="flex-col">
        Smart-contracts for keeping users' balances
      </Benefit>

      <Spacing class="h-3" />

      <Benefit class="flex-col">
        Modern web app design, most efficient and scalable solutions
      </Benefit>

      <Spacing class="h-3" />

      <Benefit class="flex-col">
        Cairo language for proving computation integrity
      </Benefit>
    </div>
  );
}
