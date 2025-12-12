/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: "#00BCD4",
        "primary-dark": "#0097A7",
        secondary: "#E91E63",
        accent: "#FFEB3B",
        dark: "#0A0E27",
        "dark-lighter": "#151A3A",
        text: "#E0E6ED",
        "text-muted": "#9CA3AF",
        "black-key": "#212121",
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
