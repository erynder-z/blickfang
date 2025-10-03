<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { lang } from "$lib/actions/lang";
  import { locale, setLocale } from "$lib/i18n";
  import LanguageMenu from "$lib/components/LanguageMenu.svelte";
  import ExifDisplaySidebar from "$lib/components/ExifDisplaySidebar.svelte";
  import OptionsDisplaySidebar from "$lib/components/OptionsDisplaySidebar.svelte";
  import colors from "$lib/colors.json";
  import { appConfig } from "$lib/store";
  import type { AppConfig } from "$lib/store";

  const setColors = async () => {
    for (const [key, value] of Object.entries(colors)) {
      document.documentElement.style.setProperty(`--color-${key}`, value as string);
    }
  };

  const getConfigData = async () => {
    let unlisten: (() => void) | undefined;

    (async () => {
      unlisten = await listen<AppConfig>("config-updated", (event) => {
        appConfig.set(event.payload);
        setLocale(event.payload.language);
      });
    })();

    return () => {
      if (unlisten) unlisten();
    };
  };

  onMount(() => {
    getConfigData();
    setColors();
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
