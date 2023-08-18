import PlusIcon from "../Icons/PlusIcon";
import Button from "./Button";

export default function NewButton(props: { onClick: () => void }) {
  return (
    <Button
      icon={PlusIcon({ size: "20px" })}
      text="New"
      height="22px"
      width="30px"
      textColor="r-light-text"
      buttonClass="m-0 p-1 px-2"
      color="bg-r-blue"
      onClick={props.onClick}
    />
  );
}
