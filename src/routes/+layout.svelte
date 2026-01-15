<script lang="ts">
  import { onMount } from "svelte";
  import { lang } from "$lib/actions/lang";
  import { locale } from "$lib/utils/i18n";
  import LanguageMenu from "$lib/components/LanguageMenu.svelte";
  import ThemeMenu from "$lib/components/ThemeMenu.svelte";
  import InfoSidebar from "$lib/components/InfoSidebar.svelte";
  import OptionsDisplaySidebar from "$lib/components/OptionsMenu.svelte";
  import { initThemeManager } from "$lib/theme/themeManager";
  import { keyboardInputManager } from "$lib/core/keyboardInputManager";
  import HotkeysMenu from "$lib/components/HotkeysMenu.svelte";
  import { appManager } from "$lib/core/appManager";
  import ToolbarMenu from "$lib/components/ToolbarMenu.svelte";
  import ImageNameDisplayMenu from "$lib/components/ImageNameDisplayMenu.svelte";
  import EdgeIndicatorMenu from "$lib/components/EdgeIndicatorMenu.svelte";
  import AppWindowMenu from "$lib/components/AppWindowMenu.svelte";
  import SaveAsMenu from "$lib/components/SaveAsMenu.svelte";
  import InitialDialog from "$lib/components/InitialDialog.svelte";
  import InitialConfigSettingsDialog from "$lib/components/InitialConfigSettingsDialog.svelte";
  import LinuxDesktopInstallDialog from "$lib/components/LinuxDesktopInstallDialog.svelte";
  import TooltipRenderer from "$lib/components/TooltipRenderer.svelte";
  import AboutMenu from "$lib/components/AboutMenu.svelte";
  import AsciiCharsMenu from "$lib/components/AsciiCharsMenu.svelte";
  import GridOverlayMenu from "$lib/components/GridOverlayMenu.svelte";

  initThemeManager();

  onMount(() => {
    let unlistenAppManager: () => void = () => {};

    appManager.initializeApp().then((unlisten) => {
      unlistenAppManager = unlisten;
    });

    return () => unlistenAppManager();
  });
</script>

<svelte:window
  on:keydown={keyboardInputManager.handleKeyDown}
  on:keyup={keyboardInputManager.handleKeyUp}
/>
<svelte:body use:lang={$locale} />

<div class="app-container">
  <HotkeysMenu />
  <LanguageMenu />
  <ThemeMenu />
  <ToolbarMenu />
  <ImageNameDisplayMenu />
  <EdgeIndicatorMenu />
  <AppWindowMenu />
  <SaveAsMenu />
  <AboutMenu />
  <AsciiCharsMenu />
  <TooltipRenderer />
  <InfoSidebar />
  <OptionsDisplaySidebar />
  <InitialDialog />
  <InitialConfigSettingsDialog />
  <LinuxDesktopInstallDialog />
  <GridOverlayMenu />

  <slot />
</div>

<style>
  .app-container {
    position: relative;
    height: 100vh;
    overflow: hidden;
  }
</style>
