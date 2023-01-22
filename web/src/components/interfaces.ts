import { JSX, JSXElement } from "solid-js";

export interface DefaultProps {
  id?: string;
  style?: JSX.CSSProperties;
  class?: string;
  children?: JSXElement;
}
