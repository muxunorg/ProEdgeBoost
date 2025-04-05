// vite.config.ts
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import Icons from 'unplugin-icons/vite'
import IconsResolver from 'unplugin-icons/resolver'
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import { ElementPlusResolver, SemiUIResolver} from 'unplugin-vue-components/resolvers'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
    plugins: [
        vue(),
        AutoImport({
            imports: [
                "vue",
                "vue-router",
                "@vueuse/core",
                {
                    "@tauri-apps/api/window": ["Window"]
                }
            ],
            dts: "types/auto-imports.d.ts",
            resolvers: [
                SemiUIResolver(), ElementPlusResolver(), IconsResolver({ prefix: 'Icon', }),],
            eslintrc: {
                enabled: true
            }
        }),
        Components({
            dirs: ["src/components/ui"],
            extensions: ["vue"],
            resolvers: [IconsResolver({ enabledCollections: ['ep'], }),ElementPlusResolver()],
            dts: "types/components.d.ts"
        }),
        Icons({
            autoInstall: true,
        })
    ],

    resolve: {
        alias: {
            "@": "/src",
            "@assets": "/src/assets",
            "@lib": "/src/lib"
        }
    },

    // Tauri专属配置
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: "ws",
                host,
                port: 1421
            }
            : undefined,
        watch: {
            ignored: ["**/src-tauri/**"]
        }
    },

    // 生产构建优化
    build: {
        target: "esnext",
        chunkSizeWarningLimit: 2000,
        rollupOptions: {
            output: {
                manualChunks: {
                    tauri: ["@tauri-apps/api"]
                }
            }
        }
    }
}));