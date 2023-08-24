export interface ICircle {
  char?: string;
  class?: string;
}

export default function Circle(props: ICircle) {
  const colors = [
    "bg-red-300",
    "bg-blue-300",
    "bg-green-300",
    "bg-yellow-300",
    "bg-pink-300",
    "bg-indigo-300",
    "bg-purple-300",
  ];
  const randomColor = colors[Math.floor(Math.random() * colors.length)];
  return (
    <div
      class={`${props.class} rounded-full grid place-items-center ${randomColor}`}
    >
      {props.char?.toUpperCase()}
    </div>
  );
}
