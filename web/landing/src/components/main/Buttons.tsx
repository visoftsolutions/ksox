import LaunchAppButton from "~/components/main/LaunchAppButton";
import LearnMoreButton from "~/components/main/LearnMoreButton";

export default function Buttons() {
  return (
    <div class="flex justify-center gap-4 max-lg:flex-col md:justify-start md:gap-6">
      <LaunchAppButton />
      <LearnMoreButton />
    </div>
  );
}
