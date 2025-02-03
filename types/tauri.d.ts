// types/tauri.d.ts
declare module '@tauri-apps/api/tauri' {
    export function invoke(command: string, args?: any): Promise<any>;
    // Add any other exports as necessary based on the Tauri API you are using
}

declare module '@tauri-apps/api' {
    export * from '@tauri-apps/api/tauri'; // This allows importing from '@tauri-apps/api'
    // Add other exports if necessary
}
