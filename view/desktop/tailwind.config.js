/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/renderer/**/*.{js,ts,jsx,tsx}'],
  darkMode: "class",
  theme: {
    extend: {}
  },
  plugins: [require('@tailwindcss/typography')]
}
