import Button from "./SideMenu/Button";
import KSOXLogo from "./SideMenu/KSOXLogo";

export default function SideMenu() {
  return (
    <div class="grid grid-rows-[100px_1fr_100px] justify-center h-full">
      <div class="row-start-1 row-end-2">
        <KSOXLogo/>
      </div>
      <div class="row-start-2 row-end-3 ">
        <Button/>
        <Button/>
        <Button/>
      </div>
      <div class="row-start-3 row-end-4 ">
        <Button/>
      </div>
    </div>
    
  );
}
