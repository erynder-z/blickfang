<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { lang } from "$lib/actions/lang";
  import { locale, setLocale } from "$lib/i18n";
  import LanguageMenu from "$lib/components/LanguageMenu.svelte";
  import ExifDisplaySidebar from "$lib/components/ExifDisplaySidebar.svelte";
  import OptionsDisplaySidebar from "$lib/components/OptionsDisplaySidebar.svelte";
  import { appConfig } from "$lib/store";
  import type { AppConfig } from "$lib/store";
  import { initThemeManager } from "$lib/themeManager";

  initThemeManager();

  onMount(() => {
    const unlistenPromise = listen<AppConfig>("config-updated", (event) => {
      appConfig.set(event.payload);
      setLocale(event.payload.language);
    });
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  });
</script>

<svelte:body use:lang={$locale} />

<LanguageMenu />

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
