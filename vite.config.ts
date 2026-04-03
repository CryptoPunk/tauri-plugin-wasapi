import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';
import { resolve } from 'node:path';

export default defineConfig({
  plugins: [
    dts({
      rollupTypes: true,
      outDir: 'dist-js',
    }),
  ],
  build: {
    lib: {
      entry: resolve(__dirname, 'guest-js/index.ts'),
      name: 'TauriPluginWasapi',
      formats: ['es', 'cjs'],
      fileName: (format) => `index.${format === 'es' ? 'js' : 'cjs'}`,
    },
    outDir: 'dist-js',
    rollupOptions: {
      external: [/^@tauri-apps\/api/],
    },
  },
});
