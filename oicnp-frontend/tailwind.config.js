/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      height: {
        800: '800px',
      },
      colors: {
        purple: {
          300: '#8a4baf',
          DEFAULT: '#663399',
          500: '#542c85',
        },
        black: {
          300: '#97a3b7',
          DEFAULT: '#48434f',
          500: '#232129',
        },
      },
    },
  },
  plugins: [],
}
