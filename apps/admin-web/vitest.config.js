import { configDefaults, defineConfig } from 'vitest/config'
import path from 'path';

export default defineConfig({
  test: {
    include: ['**/*.{test,spec}.?(c|m)[jt]s?(x)'],
    alias:  {
      '~': path.resolve(__dirname, 'src'),
    },
  },
})