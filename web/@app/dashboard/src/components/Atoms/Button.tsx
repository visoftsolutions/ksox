
export interface ButtonComponent {
    styles?: string; /* basic styles can be overriden here i.e. py-1*/
    color?: string;
    hoverColor?: string;
    textColor?: string;
    hoverTextColor?: string;
    text?: string;
    // icon?: string; // todo: add icon support
    onClick: () => void;
}

export default function Button(props: ButtonComponent) {
    return (
        <button class={` text-white font-bold py-2 px-4 rounded-full ${props.color ? `bg-${props.color}`: ""} ${props.hoverColor ? `hover:bg-${props.hoverColor}` : ""} ${props.textColor ? `text-${props.textColor}` : ""} ${props.hoverTextColor ? `hover:text-${props.hoverTextColor}` : ""} ${props.styles}`} onClick={props.onClick}>
            {/* {props.icon ? <div>
                <p>{props.text}</p>
                <img src={props.icon} alt="icon" class="h-4 w-4"/>
            </div> : <p>{props.text}</p>} */}
            {props.text}
        </button>
    );
}