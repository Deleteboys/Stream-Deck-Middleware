import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "node:path"; // 1. Path-Modul importieren

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [vue()],

    // 2. Alias hier hinzufügen
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },

    // Vite options tailored for Tauri development...
    clearScreen: false,
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
            ignored: ["**/src-tauri/**"],
        },
    },
}));