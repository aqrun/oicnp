/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    'index.html',
    './src/pages/**/*.rs',
    './src/components/**/*.rs',
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}

