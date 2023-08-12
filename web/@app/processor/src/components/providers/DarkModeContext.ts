import { createContext, useContext } from "solid-js";

export const DarkModeContext = createContext<boolean | undefined>();

export function useDarkMode() {
  return useContext(DarkModeContext);
}
