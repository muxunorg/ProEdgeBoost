/// <reference types="vite/client" />

declare module '*.vue' {
    import type { DefineComponent } from 'vue'
    const component: DefineComponent<{}, {}, any>
    export default component
}

// Tauri API 增强
interface Window {
    __TAURI__?: {
        invoke: <T>(command: string, payload?: unknown) => Promise<T>
        window: typeof import('@tauri-apps/api/window').Window
    }
}

// 环境变量类型
interface ImportMetaEnv {
    readonly VITE_APP_NAME: string
    readonly VITE_APP_VERSION: string
}

interface ImportMeta {
    readonly env: ImportMetaEnv
}