import Spacing from "../../Spacing";
import LaunchAppButton from "./LaunchAppButton";
import LearnMoreButton from "./LearnMoreButton";

export default function Buttons() {
  return (
    <div class="flex items-center justify-center max-lg:flex-col lg:gap-6">
      <LaunchAppButton />
      <Spacing class="h-4" />
      <LearnMoreButton />
    </div>
  );
}
