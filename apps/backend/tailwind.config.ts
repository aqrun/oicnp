// tailwind config is required for editor support
import type { Config } from "tailwindcss";
import sharedConfig from "@repo/tailwind-config";

const config: Pick<Config, "content" | "presets" | "darkMode" | "theme"> = {
  content: ["./src/**/*.{tsx, css}"],
  presets: [sharedConfig],
  darkMode: ['class'],
  theme: {
    container: {
      center: true,
      padding: '1.5rem',
      screens: {
        '2xl': '1360px',
      },
    },
    extend: {
      fontFamily: {
        sans: ['var(--font-sans)'],
      },
    },
  },
};

export default config;
