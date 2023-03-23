/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,jsx,ts,tsx}"],
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
      },
      fontFamily: {
        sanspro: ["Source Sans Pro", "sans-serif"],
      },
      fontSize: {
        "mainmenu-wallet": ["14px", "18px"],
        "markets-searchbar": ["14px", "18px"],
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
    screens: {
      xs: { max: "380px" },
      // => @media (max-width: 380px) { ... }
      lg: { max: "1280px" },
      // => @media (max-width: 1280px) { ... }
      xl: { max: "1440px" },
      // => @media (max-width: 1440px) { ... }
      "2xl": { max: "1920px" },
      // => @media (max-width: 1920px) { ... }
    },
  },
  plugins: [],
};
