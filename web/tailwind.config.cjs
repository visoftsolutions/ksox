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
      },
    },
  },
  plugins: [],
};
