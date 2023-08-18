const common = require("@web/tailwind/tailwind.config.cjs");

/** @type {import('tailwindcss').Config} */
module.exports = {
  ...common,
  content: [...common.content, "./node_modules/web/components/**/*.{ts,tsx}"],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        black: "#000000",
        "gray-1": "#111118",
        "gray-2": "#1B1B23",
        "gray-3": "#232330",
        "gray-4": "#7C7C8A",
        white: "#EEEEEE",
        green: "#45B780",
        red: "#B62424",
        "ksox-1": "#5532B9",
        "ksox-2": "#DA008D",

        "r-dark-background": "#000000",
        "r-dark-foreground": "#161618",
        "r-dark-active": "#111113",
        "r-dark-text": "#F4F4F4",
        "r-dark-secondary-text": "#8E8E91",
        "r-dark-search-bar": "#232325",
        "r-dark-modal-foreground": "#242426",
        "r-dark-modal-selected": "#24282F",
        "r-light-background": "#F7F7F7",
        "r-light-foreground": "#FFFFFF",
        "r-light-text": "#191C1F",
        "r-light-search-bar": "#EEEFF3",
        "r-light-modal-selected": "#F3F6FD",
        "r-blue": "#0666EB",
        "r-blue-light-backdrop": "#E6F0FD",
        "r-blue-dark-backdrop": "#1A2431",
        "r-blue-focus": "#82B2F5"
      },
      fontFamily: {
        sanspro: ["Source Sans Pro", "sans-serif"],
        lexend: ["Lexend", "sans-serif"],
      },
      fontSize: {
        navButton: ["14px", "18px"],
        searchInput: ["14px", "18px"],
        triElementHeader: ["12px", "15px"],
        triElement: ["14px", "18px"],
        stateHeader: ["12px", "15px"],
        orderbookMiddle: ["20px", "25px"],
        wallet: ["14px", "18px"],
        "markets-item": ["12px", "18px"],
        "markets-label": ["14px", "18px"],
        "markets-sublabel": ["12px", "15px"],
        "state-item": ["12px", "15px"],
        "state-label": ["14px", "18px"],
        "state-sublabel": ["12px", "15px"],
        "orderbook-item": ["12px", "18px"],
        "orderbook-label": ["14px", "18px"],
        "orderbook-sublabel": ["12px", "15px"],
        "orderbook-middle": ["20px", "25px"],
        "trades-item": ["12px", "18px"],
        "trades-label": ["14px", "18px"],
        "trades-sublabel": ["12px", "15px"],
        "submit-item": ["12px", "18px"],
        "submit-label": ["14px", "18px"],
        "submit-sublabel": ["12px", "15px"],
      },
    },
  },
  plugins: [],
};
