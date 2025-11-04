import { defineConfig } from 'vite';

export default defineConfig({
  server: {
    fs: {
      // Allow serving files from the project root
      allow: ['../../']
    }
  },
  optimizeDeps: {
    exclude: ['@achronyme/core']
  }
});
