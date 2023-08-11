import common from "@packages/tailwind/tailwind.config.js";

/** @type {import('tailwindcss').Config} */
const config = {
  ...common,
  theme: {
    extend: {
      colors: {
        primary: "#5532B9",
        secondary: "#DA008D",
        background: "#0F0D12",
        "text-1": "#EBEBEB",
        "text-2": "#776A92",
        links: "#A697C6",
        dark: "#202020",
        gradient: "#4B13EB",
        benefitscolornew: "#726CB9",
        links_new: "#A790E5",
        buttonbg_new: "#5532B9",
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
        "section-beginning": ["16px", "24px"],
        "section-title": ["24px", "36px"],
        "section-main": ["18px", "32px"],
        "section-button": ["16px", "20px"],
        "footer-title": ["32px", "48px"],
        "footer-title-light": ["18px", "32px"],
        "footer-element": ["20px", "25px"],
        "footer-element-light": ["16px", "20px"],
        "desktop-section-beginning": ["38px", "48px"],
        "hero-big-new": ["60px", "64px"],
        "hero-small-new": ["20px", "24px"],
        "text-hero-benefit-new": ["15px", "24px"],
        "section-beginning-md": ["20px", "32px"],
        "section-title-md": ["38px", "48px"],
        "section-main-md": ["20px", "32px"],
      },
    },
  },
  plugins: [],
};

export default config;
