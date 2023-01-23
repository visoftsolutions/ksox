/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,jsx,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        background: {
          landing: "#0F0D12",
        },
        primary: "#5532B9",
        secondary: "#00AB82",
        "text-white": "#EBEBEB",
        "text-faded": "#A697C6",
        "text-light": "#776A92",
      },
      fontFamily: {
        lexend: "'Lexend', sans-serif",
        inter: "'Inter', sans-serif",
      },
      fontSize: {
        logo: ["32px", "39px"],
        "main-menu-button": ["16px", "20px"],
        "hero-big": ["52px", "60px"],
        "hero-small": ["20px", "24px"],
        "hero-benefit": ["15px", "24px"],
        "hero-button": ["20px", "25px"],
        "section-beginning": ["20px", "32px"],
        "section-title": ["38px", "48px"],
        "section-main": ["20px", "32px"],
        "section-button": ["16px", "20px"],
        "footer-title": ["20px", "32px"],
        "footer-title-light": ["20px", "32px"],
        "footer-element": ["20px", "25px"],
        "footer-element-light": ["16px", "20px"],
      },
    },
  },
  plugins: [],
};
