import {
  Accessor,
  createContext,
  createSignal,
  JSX,
  useContext,
} from "solid-js";

export enum Nav {
  Landing,
  Token,
}

export const [nav, setNav] = createSignal<Nav>(Nav.Landing);
const NavContext = createContext<Accessor<Nav>>(nav);
export function NavProvider(props: { children: JSX.Element }) {
  return (
    <NavContext.Provider value={nav}>{props.children}</NavContext.Provider>
  );
}
export function useNav() {
  return useContext<Accessor<Nav>>(NavContext);
}
