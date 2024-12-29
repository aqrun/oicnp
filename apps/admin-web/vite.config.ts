import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { fileURLToPath, URL } from 'url';

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: [
      { find: '~', replacement: fileURLToPath(new URL('./src', import.meta.url)) },
    ],
  },
  server: {
    port: 8091,
    proxy: {
      '/api': {
        target: `http://localhost:${process.env.PORT}/api`,
        rewrite: path => path.replace(/^\/api/, ''),
      }
    }
  }
})
