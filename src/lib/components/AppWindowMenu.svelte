<script lang="ts">
  import { isAppWindowMenuVisible, appConfig } from "$lib/stores/appState";
  import { t } from "$lib/utils/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import { tick } from "svelte";
  import { blur } from "svelte/transition";
  import { focusTrap } from "$lib/actions/focusTrap";

  let defaultButton: HTMLButtonElement;
  let rememberButton: HTMLButtonElement;

  $: if ($isAppWindowMenuVisible) {
    tick().then(() => {
      const targetButton = $appConfig.rememberWindowSize ? rememberButton : defaultButton;
      targetButton?.focus();
    });
  }

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

  const handleClose = () => isAppWindowMenuVisible.set(false);

  const handleKeydown = (event: KeyboardEvent) => {
    if (!$isAppWindowMenuVisible) return;

    if (event.key === "Escape") handleClose();
  };
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $isAppWindowMenuVisible}
  <!-- svelte-ignore a11y-no-static-element-interactions, a11y-click-events-have-key-events -->
  <div class="backdrop" on:click={handleClose} transition:blur={{ duration: 100 }}></div>
  <div
    use:focusTrap
    class="menu-dialog"
    role="dialog"
    aria-modal="true"
    transition:blur={{ duration: 100 }}
  >
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
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: var(--color-dialog-backdrop);
    z-index: 30;
  }

  .menu-dialog {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 100;
    background-color: var(--color-background);
    border: 1px solid var(--color-accent);
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 4px 12px var(--color-shadow);
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
