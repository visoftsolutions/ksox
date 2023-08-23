import {
  Accessor,
  JSX,
  Setter,
  createContext,
  createEffect,
  createSignal,
  useContext,
} from "solid-js";

export enum ColorMode {
  Light = "Light",
  Dark = "Dark",
}

interface ColorModeContextValue {
  colorMode: Accessor<ColorMode>;
  setColorMode: Setter<ColorMode>;
}

const [colorMode, setColorMode] = createSignal<ColorMode>(ColorMode.Light);

const ColorModeContext = createContext<ColorModeContextValue>({
  colorMode,
  setColorMode,
});

export function ColorModeProvider(props: { children: JSX.Element }) {
  const localStorageKey = "colorMode";
  if (typeof localStorage !== "undefined") {
    setColorMode(
      ColorMode[
        (localStorage.getItem(localStorageKey) ??
          ColorMode.Light) as keyof typeof ColorMode
      ],
    );
  }

  createEffect(() => {
    localStorage.setItem(localStorageKey, ColorMode[colorMode()]);
  });

  return (
    <ColorModeContext.Provider value={{ colorMode, setColorMode }}>
      {props.children}
    </ColorModeContext.Provider>
  );
}
export function useColorMode() {
  return useContext<ColorModeContextValue>(ColorModeContext);
}
