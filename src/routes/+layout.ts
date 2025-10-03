// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import { invoke } from "@tauri-apps/api/core";
import { appConfig } from "$lib/store";
import type { AppConfig } from "$lib/store";
import { setLocale } from "$lib/i18n";

export const ssr = false;

export async function load() {
  try {
    const configStr: string = await invoke("read_config_command");
    let config: AppConfig = JSON.parse(configStr);
    config = {
      language: "en",
      theme: "default",
      ...config,
    };
    appConfig.set(config);
    setLocale(config.language);
  } catch (error) {
    console.error("Failed to load config:", error);
  }
}
