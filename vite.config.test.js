import { defineConfig } from 'vite';

export default defineConfig({
  root: './tests',
  server: {
    port: 8080,
    open: '/test-rust-wasm.html'
  },
  optimizeDeps: {
    exclude: ['../dist-rust/achronyme-core.mjs']
  },
  build: {
    target: 'esnext'
  }
});
