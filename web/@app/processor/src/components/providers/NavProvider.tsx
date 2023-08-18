import {
  Accessor,
  createContext,
  createSignal,
  JSX,
  useContext,
} from "solid-js";

export enum Nav {
  Home,
  Transfer,
  Settings,
  Notifications
}

export const [nav, setNav] = createSignal<Nav>(Nav.Home);
const NavContext = createContext<Accessor<Nav>>(nav);
export function NavProvider(props: { children: JSX.Element }) {
  return (
    <NavContext.Provider value={nav}>{props.children}</NavContext.Provider>
  );
}
export function useNav() {
  return useContext<Accessor<Nav>>(NavContext);
}
