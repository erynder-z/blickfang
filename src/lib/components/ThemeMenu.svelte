<script lang="ts">
  import { isThemeMenuVisible, appConfig } from "$lib/store";
  import { t } from "$lib/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import themes from "$lib/themes.json";

  const themeNames = Object.keys(themes);

  let dialog: HTMLDialogElement;

  isThemeMenuVisible.subscribe((visible) => {
    if (visible) {
      dialog?.showModal();
    } else {
      dialog?.close();
    }
  });

  const saveTheme = async (theme: string) => {
    try {
      await invoke("update_theme_command", { theme: theme });
    } catch (error) {
      console.error("Failed to save theme:", error);
    }
  };

  const handleButtonClick = (theme: string) => {
    saveTheme(theme);
    handleClose();
  };

  const handleClose = () => {
    isThemeMenuVisible.set(false);
  };
</script>

<dialog bind:this={dialog} on:close={handleClose}>
  <div class="menu-content">
    <h1>{$t["options.theme.heading"]}</h1>

    {#each themeNames as theme}
      <!-- svelte-ignore a11y_autofocus -->
      <button on:click={() => handleButtonClick(theme)} autofocus={$appConfig.theme === theme}>
        {theme}
      </button>
    {/each}

    <button on:click={handleClose} class="close-button">{$t["options.button.close"]}</button>
  </div>
</dialog>

<style>
  dialog {
    background-color: var(--color-background);
    border: 1px solid var(--color-accent);
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 4px 12px var(--color-shadow);
  }

  dialog::backdrop {
    background: var(--color-accent);
  }

  .menu-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    min-width: 200px;
    text-align: center;
  }

  h1 {
    margin: 0 0 0.5rem 0;
    color: #e3e3e3;
  }

  button {
    padding: 0.5rem;
    border: none;
    border-radius: 0.5rem;
    background-color: var(--color-button);
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: bold;
    text-transform: capitalize;
  }

  .close-button {
    margin-top: 1rem;
  }
</style>
