<script lang="ts">
  import { isImageNameDisplayMenuVisible, appConfig } from "$lib/store";
  import { t } from "$lib/i18n";
  import { invoke } from "@tauri-apps/api/core";

  type DisplayMode = "show" | "hide" | "fade";

  const modes: DisplayMode[] = ["show", "hide", "fade"];

  let dialog: HTMLDialogElement;
  let buttons: HTMLButtonElement[] = [];

  isImageNameDisplayMenuVisible.subscribe((visible) => {
    if (visible) {
      dialog?.showModal();
      const currentIndex = modes.indexOf($appConfig.imageNameDisplayMode);
      if (currentIndex !== -1) buttons[currentIndex]?.focus();
    } else {
      dialog?.close();
    }
  });

  const saveDisplayMode = async (mode: DisplayMode) => {
    try {
      await invoke("update_image_name_display_mode_command", { mode });
    } catch (error) {
      console.error("Failed to save display mode:", error);
    }
  };

  const handleButtonClick = (mode: DisplayMode) => {
    appConfig.update((config) => ({ ...config, imageNameDisplayMode: mode }));
    saveDisplayMode(mode);
    handleClose();
  };

  const handleClose = () => {
    isImageNameDisplayMenuVisible.set(false);
  };

  const getLabel = (mode: DisplayMode) => {
    if (mode === "show" || mode === "hide") {
      return `general.${mode}`;
    }
    return `image_name_display.option.${mode}`;
  };
</script>

<dialog bind:this={dialog} on:close={handleClose}>
  <div class="menu-content">
    <h1>{$t["image_name_display.title"]}</h1>

    {#each modes as mode, i}
      <button bind:this={buttons[i]} on:click={() => handleButtonClick(mode)}>
        {$t[getLabel(mode)]}
      </button>
    {/each}

    <button on:click={handleClose} class="close-button">{$t["general.close"]}</button>
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
    background: var(--color-dialog-backdrop);
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
  }

  button:focus {
    outline: none;
    background-color: var(--color-accent);
  }

  .close-button {
    margin-top: 1rem;
  }
</style>
