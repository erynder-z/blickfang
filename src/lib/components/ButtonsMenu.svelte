<script lang="ts">
  import { isButtonMenuVisible, appConfig } from "$lib/store";
  import { t } from "$lib/i18n";
  import { invoke } from "@tauri-apps/api/core";

  const buttonSizes = ["large", "small", "hidden"];

  let dialog: HTMLDialogElement;

  isButtonMenuVisible.subscribe((visible) => {
    if (visible) {
      dialog?.showModal();
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
</script>

<dialog bind:this={dialog} on:close={handleClose}>
  <div class="menu-content">
    <h1>{$t["options.buttons.heading"]}</h1>

    {#each buttonSizes as size}
      <!-- svelte-ignore a11y_autofocus -->
      <button on:click={() => handleButtonClick(size)} autofocus={$appConfig.buttonSize === size}>
        {$t[`options.UI_buttons.${size}`]}
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

  .close-button {
    margin-top: 1rem;
  }
</style>
