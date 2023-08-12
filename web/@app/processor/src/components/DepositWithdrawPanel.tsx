import Button from "./Atoms/Buttons/Button";
import ButtonTile from "./Atoms/Buttons/ButtonTile";
import DepositIcon from "./Atoms/Icons/DepositIcon";
import WithdrawIcon from "./Atoms/Icons/WithdrawIcon";
import { Palette } from "./Atoms/Palette";


export default function DepositWithdrawPanel() {

    const depositIcon = DepositIcon({stroke: Palette["r-blue"], size: "26px"});
    const withdrawIcon = WithdrawIcon({stroke: Palette["r-blue"], size: "26px"});

    return (
        <div class="flex flex-row justify-center">
            <ButtonTile button={{icon: DepositIcon({ stroke: Palette["r-blue"], size: "22px" })}} text="Deposit" />
            <ButtonTile button={{icon: WithdrawIcon({ stroke: Palette["r-blue"], size: "22px" })}} text="Withdraw" />
        </div>
    )
}