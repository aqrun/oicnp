module.exports = {
  // prefix: 'tw-',
  content: ["./app/**/*.{ts,tsx}"],
  theme: {
    extend: {
      backgroundImage: {
        'login-bg': "url('/images/login-new.jpeg')",
      }
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}
