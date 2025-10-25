// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info

import { appConfig, type AppConfig } from "$lib/stores/appState";
import { setLocale } from "$lib/utils/i18n";
import { browser } from "$app/environment";
import type { LoadEvent } from "@sveltejs/kit";

export const ssr = false;

export async function load({ fetch }: LoadEvent) {
  if (browser) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const configStr: string = await invoke("read_config_command");
      const config: AppConfig = JSON.parse(configStr);
      appConfig.set(config);
      setLocale(config.language);
    } catch (error) {
      console.error("Failed to load config:", error);
    }
  }
}
