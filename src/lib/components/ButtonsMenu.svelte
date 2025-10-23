<script lang="ts">
  import { isButtonMenuVisible, appConfig } from "$lib/store";
  import { t } from "$lib/i18n";
  import { invoke } from "@tauri-apps/api/core";

  const buttonSizes = ["large", "small", "hide"];

  let dialog: HTMLDialogElement;
  let buttons: HTMLButtonElement[] = [];

  isButtonMenuVisible.subscribe((visible) => {
    if (visible) {
      dialog?.showModal();
      const currentIndex = buttonSizes.indexOf($appConfig.buttonSize);
      if (currentIndex !== -1) buttons[currentIndex]?.focus();
    } else {
      dialog?.close();
    }
  });

  const saveButtonSize = async (size: string) => {
    try {
      await invoke("update_button_size_command", { buttonSize: size });
    } catch (error) {
      console.error("Failed to save button size:", error);
    }
  };

  const handleButtonClick = (size: string) => {
    saveButtonSize(size);
    handleClose();
  };

  const handleClose = () => {
    isButtonMenuVisible.set(false);
  };

  const getLabel = (size: string) => {
    if (size === "large" || size === "small") {
      return `general.${size}`;
    }
    return `general.${size}`;
  };
</script>

<dialog bind:this={dialog} on:close={handleClose}>
  <div class="menu-content">
    <h1>{$t["options.button.heading"]}</h1>

    {#each buttonSizes as size, i}
      <button bind:this={buttons[i]} on:click={() => handleButtonClick(size)}>
        {$t[getLabel(size)]}
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
    color: var(--color-text-primary);
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

  button:focus {
    outline: none;
    background-color: var(--color-accent);
  }

  .close-button {
    margin-top: 1rem;
  }
</style>
