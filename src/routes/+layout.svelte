<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { lang } from "$lib/actions/lang";
  import { locale } from "$lib/i18n";
  import LanguageMenu from "$lib/components/LanguageMenu.svelte";
  import ExifDisplaySidebar from "$lib/components/ExifDisplaySidebar.svelte";
  import OptionsDisplaySidebar from "$lib/components/OptionsDisplaySidebar.svelte";

  onMount(async () => {
    const config = await invoke("read_config_command");
    console.log(config);
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
