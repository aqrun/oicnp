import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { fileURLToPath, URL } from 'url';
import bodyParser from 'body-parser';
// import cookieParser from 'cookie-parser'
import mockServer from 'vite-plugin-mock-server';

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    mockServer({
      logLevel: 'info',
      urlPrefixes: [ '/api/' ],
      mockRootDir: './mocks',
      middlewares: [
        // cookieParser(),
        bodyParser.json(),
        bodyParser.urlencoded(),
        bodyParser.text(),
        bodyParser.raw()
      ]
    })
  ],
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
