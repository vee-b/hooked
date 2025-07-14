import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

// // @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [sveltekit()],

  optimizeDeps: {
    include: ['uuid', 'date-fns', '@tauri-apps/api'] // Ensure these are pre-bundled if needed
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
    allowedHosts: ['b21a-82-41-69-161.ngrok-free.app'],
  },
}));
