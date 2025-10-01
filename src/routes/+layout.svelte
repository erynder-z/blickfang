<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { lang } from "$lib/actions/lang";
  import { locale } from "$lib/i18n";
  import LanguageMenu from "$lib/components/LanguageMenu.svelte";
  import ExifDisplaySidebar from "$lib/components/ExifDisplaySidebar.svelte";
  import OptionsDisplaySidebar from "$lib/components/OptionsDisplaySidebar.svelte";
  import colors from "$lib/colors.json";

  const getConfig = async () => {
    const config = await invoke("read_config_command");
    console.log(config);
  };

  const setColors = async () => {
    for (const [key, value] of Object.entries(colors)) {
      document.documentElement.style.setProperty(`--color-${key}`, value as string);
    }
  };

  onMount(async () => {
    getConfig();
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
