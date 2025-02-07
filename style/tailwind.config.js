/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        "bg": "#030303",
        "accent": "#4032a8",
        "accent-light": "#7851ed",
        "text": "#e0e0e0",
        "bg-2": "#212121",
        "accent-border": "#7851ed",
        "error": "red",
      }
    },
  },
  plugins: [],
}
