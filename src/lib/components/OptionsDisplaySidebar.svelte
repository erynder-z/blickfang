<script lang="ts">
  import { t } from "$lib/i18n";
  import { isLanguageMenuVisible, isOptionsSidebarVisible } from "$lib/store";

  const showLanguageSelectOverlay = () => {
    isLanguageMenuVisible.set(true);
  };
</script>

<div class="options-sidebar-overlay" class:visible={$isOptionsSidebarVisible}>
  <div class="options-container">
    <h1>{$t["options.title"]}</h1>
    <button>{$t["options.button.hotkeys"]}</button>
    <button on:click={showLanguageSelectOverlay}>{$t["options.button.language"]}</button>
  </div>
</div>

<style>
  .options-sidebar-overlay {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 25%;
    max-width: 33%;
    min-width: 25%;
    background-color: #372e49;
    transform: translateX(-100%);
    transition: transform 0.3s ease-in-out;
    z-index: 20;
    box-shadow: 2px 0 5px rgba(0, 0, 0, 0.5);
    border-right: 1px solid #444;
  }

  .options-sidebar-overlay.visible {
    transform: translateX(0);
  }

  .options-container {
    padding: 1rem;
    color: #e3e3e3;
    height: 100%;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  h1 {
    margin: 0;
    text-align: center;
  }

  button {
    padding: 0.5rem;
    border: none;
    border-radius: 0.5rem;
    background-color: rgba(255, 255, 255, 0.2);
    color: #e3e3e3;
    cursor: pointer;
    font-weight: bold;
  }
</style>
