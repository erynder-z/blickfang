<script lang="ts">
  import { onMount } from "svelte";
  import { lang } from "$lib/actions/lang";
  import { locale } from "$lib/utils/i18n";
  import LanguageMenu from "$lib/components/LanguageMenu.svelte";
  import ThemeMenu from "$lib/components/ThemeMenu.svelte";
  import InfoSidebar from "$lib/components/InfoSidebar.svelte";
  import OptionsDisplaySidebar from "$lib/components/OptionsMenu.svelte";
  import { initThemeManager } from "$lib/theme/themeManager";
  import { handleKeyDown, handleKeyUp } from "$lib/core/keyboardInputManager";
  import HotkeysMenu from "$lib/components/HotkeysMenu.svelte";
  import { initializeApp } from "$lib/core/appManager";
  import ButtonsMenu from "$lib/components/ButtonsMenu.svelte";
  import ImageNameDisplayMenu from "$lib/components/ImageNameDisplayMenu.svelte";
  import EdgeIndicatorMenu from "$lib/components/EdgeIndicatorMenu.svelte";
  import AppWindowMenu from "$lib/components/AppWindowMenu.svelte";

  initThemeManager();

  onMount(() => {
    const unlisten = initializeApp();

    return () => unlisten();
  });
</script>

<svelte:window on:keydown={handleKeyDown} on:keyup={handleKeyUp} />
<svelte:body use:lang={$locale} />

<HotkeysMenu />
<LanguageMenu />
<ThemeMenu />
<ButtonsMenu />
<ImageNameDisplayMenu />
<EdgeIndicatorMenu />
<AppWindowMenu />

<div class="app-container">
  <InfoSidebar />
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
