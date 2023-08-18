import Picture from "./Picture";

export interface IProfile {
  name: string;
  img: string;
  className?: string;
}

export default function Profile(props: IProfile) {
  return (
    <div class={`flex flex-row items-center ${props.className}`}>
      <Picture src={props.img} size={32}/>
      <p class="ml-4">{props.name}</p>
    </div>
  );
}
