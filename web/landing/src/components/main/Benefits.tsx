import Spacing from "~/components/Spacing";
import Benefit from "~/components/main/Benefit";

export default function Benefits() {
  return (
    <div class="text-benefitsnew flex flex-col items-center md:items-start md:text-text-1  lg:items-start lg:pb-11 ">
      <Benefit>Smart-contracts for keeping users' balances</Benefit>

      <Spacing class="h-2" />

      <Benefit>
        Modern web app design, most efficient and scalable solutions
      </Benefit>

      <Spacing class="h-2" />

      <Benefit>Cairo language for proving computation integrity</Benefit>
    </div>
  );
}
