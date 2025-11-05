import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait()
  ],
  server: {
    fs: {
      // Allow serving files from the project root
      allow: ['../../']
    }
  },
  optimizeDeps: {
    // Remove exclusion to let Vite bundle from node_modules
    // exclude: ['@achronyme/core']
  }
});
