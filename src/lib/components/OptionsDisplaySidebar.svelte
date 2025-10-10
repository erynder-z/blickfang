<script lang="ts">
  import { t } from "$lib/i18n";
  import {
    isLanguageMenuVisible,
    isOptionsSidebarVisible,
    isThemeMenuVisible,
    isHotkeysMenuVisible,
  } from "$lib/store";

  const showLanguageSelectOverlay = () => {
    isLanguageMenuVisible.set(true);
  };

  const showThemeSelectOverlay = () => {
    isThemeMenuVisible.set(true);
  };

  const showHotkeysOverlay = () => {
    isHotkeysMenuVisible.set(true);
  };
</script>

<div class="options-sidebar-overlay" class:visible={$isOptionsSidebarVisible}>
  <div class="options-container">
    <h1>{$t["options.title"]}</h1>
    <button on:click={showHotkeysOverlay}>{$t["options.button.hotkeys"]}</button>
    <button on:click={showLanguageSelectOverlay}>{$t["options.button.language"]}</button>
    <button on:click={showThemeSelectOverlay}>{$t["options.button.theme"]}</button>
  </div>
</div>

<style>
  .options-sidebar-overlay {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 25%;
    max-width: 30ch;
    background-color: var(--color-background);
    transform: translateX(-100%);
    transition: transform 0.3s ease-in-out;
    z-index: 20;
    box-shadow: 2px 0 5px var(--color-shadow);
    border-right: 1px solid var(--color-accent);
  }

  .options-sidebar-overlay.visible {
    transform: translateX(0);
  }

  .options-container {
    color: var(--color-text-primary);
    height: 100%;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
  }

  h1 {
    text-align: center;
  }

  button {
    padding: 0.5rem;
    border: none;
    border-radius: 0.5rem;
    background-color: var(--color-button);
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: bold;
  }
</style>
