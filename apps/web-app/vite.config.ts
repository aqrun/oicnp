import { defineConfig, type PluginOption } from 'vite';
import react from '@vitejs/plugin-react-swc';
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss() as unknown as PluginOption],
  build: {
    // 在 outDir 中生成 .vite/manifest.json
    manifest: true,
    cssCodeSplit: false, // 将所有 CSS 打包到一个文件中，方便在模板中引用
    rollupOptions: {
      // 覆盖默认的 .html 入口
      input: {
        // index: './index.html',
        main: 'src/main.ts',
        // style: 'src/index.css',
      },
      // output: {
      //   // 开发环境使用固定文件名，生产环境使用 hash
      //   entryFileNames: process.env.NODE_ENV === 'production' 
      //     ? '[name]-[hash].js' 
      //     : '[name].js',
      //   chunkFileNames: process.env.NODE_ENV === 'production' 
      //     ? '[name]-[hash].js' 
      //     : '[name].js',
      //   assetFileNames: process.env.NODE_ENV === 'production' 
      //     ? '[name]-[hash].[ext]' 
      //     : '[name].[ext]',
      // },
    },
  },
  server: {
    hmr: {
      port: 21012,
    },
  },
})
