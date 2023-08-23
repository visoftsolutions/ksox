import Picture from "~/components/Atoms/Picture";

export interface IProfile {
  name: string;
  img: string;
  class?: string;
}

export default function Profile(props: IProfile) {
  return (
    <div class={`flex flex-row items-center ${props.class}`}>
      <Picture src={props.img} size={32} />
      <p class="ml-4">{props.name}</p>
    </div>
  );
}
