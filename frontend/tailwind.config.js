/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: ["./src/**/*.{rs,html,css}", "./docs/**/*.html"],
    theme: {
      extend: {
        height: {
            // 58px
            58: '3.625rem',
            800: '50rem',
        },
        lineHeight: {
            58: '3.625rem',
        },
        padding: {
            58: '3.625rem',
        },
        colors: {
            purple: {
                300: '#8a4baf',
                DEFAULT: '#663399',
                500: '#542c85',
                800: '#0f0226',
                900: '#11081f',
            },
            black: {
                300: '#97a3b7',
                DEFAULT: '#48434f',
                500: '#232129',
            },
        },
        fontFamily: {
          sans: ["Lexend", "sans-serif"],
          // sans: ["Inter var", "sans-serif"],
        },
        boxShadow: {
          "3xl": "0 35px 60px -1ww5px rgba(0, 0, 0, 0.5)",
          cutesy: "0px 0px 40px -5px rgba(255, 255, 255, 0.2)",
          // cutesy: "0px 0px 30px -10px white",
          // cutesy: "0px 0px 30px -10px red",
          pop: "0px 0px 30px -10px rgba(0, 0, 0, 0.5)",
        },
        keyframes: {
          fadein: {
            'from': { opacity: '0' },
            'to': { opacity: '1' },
          }
        },
        animation: {
          'fadein-medium': 'fadein 500ms ease-in-out forwards',
        },
      },
    },
    plugins: [],
  };
  
