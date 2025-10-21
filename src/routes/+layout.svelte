<script lang="ts">
  import { onMount } from "svelte";
  import { lang } from "$lib/actions/lang";
  import { locale } from "$lib/i18n";
  import LanguageMenu from "$lib/components/LanguageMenu.svelte";
  import ThemeMenu from "$lib/components/ThemeMenu.svelte";
  import InfoSidebar from "$lib/components/InfoSidebar.svelte";
  import OptionsDisplaySidebar from "$lib/components/OptionsDisplaySidebar.svelte";
  import { initThemeManager } from "$lib/themeManager";
  import { handleKeyDown, handleKeyUp } from "$lib/keyboardInputManager";
  import HotkeysMenu from "$lib/components/HotkeysMenu.svelte";
  import { initializeApp } from "$lib/appManager";
  import ButtonsMenu from "$lib/components/ButtonsMenu.svelte";
  import ImageNameDisplayMenu from "$lib/components/ImageNameDisplayMenu.svelte";

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
