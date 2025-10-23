<script lang="ts">
  import { isEdgeIndicatorMenuVisible, appConfig } from "$lib/store";
  import { t } from "$lib/i18n";
  import { invoke } from "@tauri-apps/api/core";

  let dialog: HTMLDialogElement;
  let showButton: HTMLButtonElement;
  let hideButton: HTMLButtonElement;

  isEdgeIndicatorMenuVisible.subscribe((visible) => {
    if (!visible) {
      dialog?.close();
      return;
    }

    dialog?.showModal();

    const targetButton = $appConfig.edgeIndicatorsVisible ? showButton : hideButton;

    targetButton?.focus();
  });

  const saveEdgeIndicatorMode = async (visible: boolean) => {
    try {
      await invoke("update_edge_indicators_visible_command", { visible });
    } catch (error) {
      console.error("Failed to save edge indicator mode:", error);
    }
  };

  const handleButtonClick = (visible: boolean) => {
    saveEdgeIndicatorMode(visible);
    handleClose();
  };

  const handleClose = () => {
    isEdgeIndicatorMenuVisible.set(false);
  };
</script>

<dialog bind:this={dialog} on:close={handleClose}>
  <div class="menu-content">
    <h1>{$t["options.button.edge_indicators"]}</h1>

    <button bind:this={showButton} on:click={() => handleButtonClick(true)}>
      {$t["general.show"]}
    </button>

    <button bind:this={hideButton} on:click={() => handleButtonClick(false)}>
      {$t["general.hide"]}
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
