import Picture from "./Atoms/Picture";

export interface ITransfer {
    name: string;
    img: string;
    date: string; // i.e. Aug 21, Apr 11, 2021
    text: string;
}

export default function Transfer(props: ITransfer) {
    
        return (<div
            class="rounded-xl bg-r-light-foreground dark:bg-r-dark-foreground active:bg-r-light-background dark:active:bg-r-dark-active"
            // onClick={props.onClick || (() => {})}
          >
            <div class="flex justify-between">
        <div class="m-4 flex">
          <Picture src={props.img} alt="test" size={42} />
          <div class="ml-4">
            <p class="text-dark-text font-sans font-bold ">{props.name}</p>
            <p class="font-sans text-xs text-r-dark-secondary-text">{props.text}</p>
          </div>
        </div>
        <div class="m-4 flex flex-col items-end">
          <p class="font-sans text-xs text-r-dark-secondary-text">
            {props.date}
          </p>
        </div>
      </div>
        </div>)
}