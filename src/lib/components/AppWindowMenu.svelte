<script lang="ts">
  import { isAppWindowMenuVisible, appConfig } from "$lib/store";
  import { t } from "$lib/i18n";
  import { invoke } from "@tauri-apps/api/core";

  let dialog: HTMLDialogElement;
  let defaultButton: HTMLButtonElement;
  let rememberButton: HTMLButtonElement;

  isAppWindowMenuVisible.subscribe((visible) => {
    if (visible) {
      dialog?.showModal();
      const targetButton = $appConfig.rememberWindowSize ? rememberButton : defaultButton;
      targetButton?.focus();
    } else {
      dialog?.close();
    }
  });

  const saveRememberWindowSize = async (remember: boolean) => {
    try {
      await invoke("update_remember_window_size_command", { remember });
    } catch (error) {
      console.error("Failed to save remember window size:", error);
    }
  };

  const handleButtonClick = (remember: boolean) => {
    saveRememberWindowSize(remember);
    handleClose();
  };

  const handleClose = () => {
    isAppWindowMenuVisible.set(false);
  };
</script>

<dialog bind:this={dialog} on:close={handleClose}>
  <div class="menu-content">
    <h1>{$t["app_window.title"]}</h1>

    <button bind:this={defaultButton} on:click={() => handleButtonClick(false)}>
      {$t["app_window.option.default"]}
    </button>

    <button bind:this={rememberButton} on:click={() => handleButtonClick(true)}>
      {$t["app_window.option.remember"]}
    </button>

    <button on:click={handleClose} class="close-button">
      {$t["general.close"]}
    </button>
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
    background: var(--color-dialog-backdrop );
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
  }

  button:focus {
    outline: none;
    background-color: var(--color-accent);
  }

  .close-button {
    margin-top: 1rem;
  }
</style>
