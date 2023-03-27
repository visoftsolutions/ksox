import LaunchAppButton from "./LaunchAppButton";
import LearnMoreButton from "./LearnMoreButton";

export default function Buttons() {
  return (
    <div class="flex justify-center gap-4 max-lg:flex-col md:justify-start md:gap-6">
      <LaunchAppButton />
      <LearnMoreButton />
    </div>
  );
}
