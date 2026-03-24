import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { readFileSync } from 'fs';

const pkg = JSON.parse(readFileSync('package.json', 'utf-8'));

export default defineConfig({
  plugins: [svelte()],
  define: {
    __APP_VERSION__: JSON.stringify(pkg.version),
  },
  resolve: {
    conditions: ['browser'],
  },
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./src/test/setup.ts'],
    include: ['src/**/*.test.ts'],
    coverage: {
      provider: 'v8',
      include: ['src/lib/**'],
    },
  },
});
