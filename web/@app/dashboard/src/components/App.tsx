import BadgePanel from "./BadgePanel";
import badgeData from "./badgeData.json"

export default function App() {
  return <div class="flex justify-center items-center">
    <BadgePanel badgeData={badgeData}></BadgePanel>
  </div>
}
