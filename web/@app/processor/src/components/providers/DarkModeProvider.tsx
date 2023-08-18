import {
  Accessor,
  Setter,
  createContext,
  createEffect,
  createSignal,
  useContext,
} from "solid-js";

interface DarkModeValue {
  darkMode: Accessor<boolean>;
  setDarkMode: Setter<boolean>;
}

const DarkModeContext = createContext<DarkModeValue>();

export function DarkModeProvider(props: any) {
  const localStorageKey = "darkMode";
  const initialDarkMode = (() => {
    if (typeof localStorage !== "undefined") {
      return localStorage.getItem(localStorageKey) === "true" ? true : false;
    }
    // Return default value (e.g., based on system preference) if localStorage is not available
    return false;
  })();

  const [darkMode, setDarkMode] = createSignal<boolean>(initialDarkMode);

  // Watch for changes to the darkMode signal and update local storage
  createEffect(() => {
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(localStorageKey, darkMode() ? "true" : "false");
    } 
  });

  const darkModeValue: DarkModeValue = {
    darkMode: darkMode,
    setDarkMode: setDarkMode,
  };

  return (
    <DarkModeContext.Provider value={darkModeValue}>
      {props.children}
    </DarkModeContext.Provider>
  );
}

export const useDarkModeContext = () => useContext(DarkModeContext)!;
