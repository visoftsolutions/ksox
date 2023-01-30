/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,jsx,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        primary: "#5532B9",
        secondary: "#00AB82",
        background: "#0F0D12",
        "text-1": "#EBEBEB",
        "text-2": "#776A92",
        links: "#A697C6",
        dark: "#202020",
        gradient: "#5532B9",
      },
      fontFamily: {
        lexend: "'Lexend', sans-serif",
        inter: "'Inter', sans-serif",
      },
      fontSize: {
        logo: ["24px", "29px"],
        "main-menu-button": ["16px", "20px"],
        "hero-big": ["32px", "42px"],
        "hero-small": ["18px", "24px"],
        "hero-benefit": ["14px", "24px"],
        "hero-button": ["20px", "25px"],
        "section-beginning": ["20px", "32px"],
        "section-title": ["38px", "48px"],
        "section-main": ["20px", "32px"],
        "section-button": ["16px", "20px"],
        "footer-title": ["32px", "48px"],
        "footer-title-light": ["18px", "32px"],
        "footer-element": ["20px", "25px"],
        "footer-element-light": ["16px", "20px"],
      },
    },
  },
  plugins: [],
};
