import ProgressBar from "@app/exchange-landing/src/components/Token/ProgressBar"

export interface BadgeProps {
    name: string;
    description: string;
    imageURL: string;
    progress: number;
}

export default function Badge(props: BadgeProps) {
    return <>
      <div 
        class="p-4 w-48 h-72 flex flex-col items-center justify-center bg-gray-2 rounded-xl"
        style={{ opacity: props.progress == 1 ? 1 : 0.4 }}
      >
        <img class={`w-2/3 m-4 h-auto transition-transform ${props.progress >= 1 ? "hover:scale-125" : ""}`} src={props.imageURL}></img>
        <p class="text-center text-xl font-bold">{props.name}</p>
        <p class="text-center text-gray-300 mb-4">{props.description}</p>

        <ProgressBar fill={props.progress} disable={false}></ProgressBar>
      </div>
    </>
}