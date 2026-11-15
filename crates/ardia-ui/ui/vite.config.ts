import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@components': path.resolve(__dirname, './src/components'),
      '@state': path.resolve(__dirname, './src/state'),
      '@platform': path.resolve(__dirname, './src/platform'),
    },
  },
  server: {
    port: 1420,
    strictPort: true,
    hmr: {
      port: 1421,
    },
  },
  build: {
    outDir: 'dist',
    sourcemap: true,
    minify: 'esbuild',
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom', 'zustand'],
          tauri: ['@tauri-apps/api', '@tauri-apps/plugin-clipboard-manager', '@tauri-apps/plugin-dialog', '@tauri-apps/plugin-fs', '@tauri-apps/plugin-opener', '@tauri-apps/plugin-shell', '@tauri-apps/plugin-store', '@tauri-apps/plugin-updater'],
        },
      },
    },
  },
});