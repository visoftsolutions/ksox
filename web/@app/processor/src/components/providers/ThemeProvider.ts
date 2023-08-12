// ThemeProvider.tsx
import { createThemeStore } from "./state";
import { DarkModeContext } from "./DarkModeContext";

export const ThemeProvider: React.FC = ({ children }) => {
  const themeStore = createThemeStore(); // Create your theme store here

  return (
    <DarkModeContext.Provider value={themeStore.isDarkMode}>
      {children}
    </DarkModeContext.Provider>
  );
};
