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
        white: "#FFFFFF",
        green: "#45B780",
        red: "#B62424",
        "ksox-1": "#5532B9",
        "ksox-2": "#DA008D",
      },
      fontFamily: {
        sanspro: ["Source Sans Pro", "sans-serif"],
      },
      fontSize: {
        "assets-searchbar": ["14px", "18px"],
        "assets-item": ["14px", "18px"],
        "assets-label": ["14px", "18px"],
        "assets-sublabel": ["12px", "15px"],
        "openorders-item": ["14px", "18px"],
        "openorders-label": ["14px", "18px"],
        "openorders-sublabel": ["12px", "15px"],
        "orderbook-item": ["14px", "18px"],
        "orderbook-label": ["14px", "18px"],
        "orderbook-sublabel": ["12px", "15px"],
        "orderbook-middle": ["20px", "25px"],
        "trades-item": ["14px", "18px"],
        "trades-label": ["14px", "18px"],
        "trades-sublabel": ["12px", "15px"],
      },
    },
    screens: {
      "xs-max": { max: "380px" },
      // => @media (max-width: 380px) { ... }
      "lg-max": { max: "1280px" },
      // => @media (max-width: 1280px) { ... }
      "xl-max": { max: "1440px" },
      // => @media (max-width: 1440px) { ... }
      "2xl-max": { max: "1920px" },
      // => @media (max-width: 1920px) { ... }
      "xs-min": { min: "380px" },
      // => @media (min-width: 380px) { ... }
      "lg-min": { min: "1280px" },
      // => @media (min-width: 1280px) { ... }
      "xl-min": { min: "1440px" },
      // => @media (min-width: 1440px) { ... }
      "2xl-min": { min: "1920px" },
      // => @media (min-width: 1920px) { ... }
    },
  },
  plugins: [],
};
