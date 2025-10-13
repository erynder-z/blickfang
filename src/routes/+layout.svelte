<script lang="ts">
  import { onMount } from "svelte";
  import { get } from "svelte/store";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { lang } from "$lib/actions/lang";
  import { locale, setLocale } from "$lib/i18n";
  import LanguageMenu from "$lib/components/LanguageMenu.svelte";
  import ThemeMenu from "$lib/components/ThemeMenu.svelte";
  import ExifDisplaySidebar from "$lib/components/ExifDisplaySidebar.svelte";
  import OptionsDisplaySidebar from "$lib/components/OptionsDisplaySidebar.svelte";
  import { appConfig, isRemapping } from "$lib/store";
  import type { AppConfig } from "$lib/store";
  import { initThemeManager } from "$lib/themeManager";
  import { handleKeyDown as originalHandleKeyDown } from "$lib/shortcuts";
  import HotkeysMenu from "$lib/components/HotkeysMenu.svelte";

  initThemeManager();

  onMount(() => {
    let unlistenImageSource: () => void;
    let unlistenConfig: () => void;

    (async () => {
      unlistenImageSource = await listen<string[]>("image-source", (event) => {
        console.log("Paths received:", event.payload);
      });

      await invoke("frontend_is_ready");

      unlistenConfig = await listen<AppConfig>("config-updated", (event) => {
        appConfig.set(event.payload);
        setLocale(event.payload.language);
      });
    })();

    // synchronous cleanup function
    return () => {
      unlistenImageSource?.();
      unlistenConfig?.();
    };
  });

  const handleGlobalKeyDown = (event: KeyboardEvent) => {
    if (get(isRemapping)) return;

    originalHandleKeyDown(event);
  };
</script>

<svelte:window on:keydown={handleGlobalKeyDown} />
<svelte:body use:lang={$locale} />

<HotkeysMenu />
<LanguageMenu />
<ThemeMenu />

<div class="app-container">
  <ExifDisplaySidebar />
  <OptionsDisplaySidebar />

  <slot />
</div>

<style>
  .app-container {
    position: relative;
    height: 100vh;
    overflow: hidden;
  }
</style>
