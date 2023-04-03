import Markets from "~/components/Markets";
import CreateChart from "~/components/Chart";
import MainMenu from "~/components/MainMenu";
import { useSession } from "~/components/Buttons/WalletButton";

export default function Assets() {
  const session = useSession();
  const precision = 3;
  const capacity = 20;

  return (
    <>
      <div class="col-start-2 col-end-6 row-start-2 row-end-4 bg-red">
        assets
      </div>
    </>
  );
}
