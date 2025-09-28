<script lang="ts">
  import { t } from "$lib/i18n";
  import { isLanguageMenuVisible } from "$lib/store";

  const showLanguageSelectOverlay = () => {
    isLanguageMenuVisible.set(true);
  };
</script>

<div class="options-container">
  <h1>{$t["options.title"]}</h1>
  <button>{$t["options.button.hotkeys"]}</button>
  <button on:click={showLanguageSelectOverlay}>{$t["options.button.language"]}</button>
</div>

<style>
  .options-container {
    padding: 1rem;
    color: #e3e3e3;
    height: 100%;
    overflow-y: auto;
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
