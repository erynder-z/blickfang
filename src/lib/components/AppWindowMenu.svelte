<script lang="ts">
  import { isAppWindowMenuVisible, appConfig } from "$lib/stores";
  import { t } from "$lib/utils/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import { tick } from "svelte";
  import { fly, fade } from "svelte/transition";
  import { focusTrap } from "$lib/actions/focusTrap";

  let defaultButton: HTMLButtonElement;
  let rememberButton: HTMLButtonElement;

  $: if ($isAppWindowMenuVisible) {
    tick().then(() => {
      const targetButton = $appConfig.rememberWindowSize ? rememberButton : defaultButton;
      targetButton?.focus();
    });
  }

  /**
   * Save the remember window size setting.
   * @param {boolean} remember - Whether to remember window size and position.
   * @returns {Promise<void>} - A promise that resolves when the setting is saved.
   */
  const saveRememberWindowSize = async (remember: boolean) => {
    try {
      await invoke("update_remember_window_size_command", { remember });
    } catch (error) {
      console.error("Failed to save remember window size:", error);
    }
  };

  /**
   * Handles the button click event for the remember window size setting.
   * Saves the remember window size setting to the app config.
   * @param {boolean} remember - Whether to remember window size and position.
   * @returns {Promise<void>} - A promise that resolves when the setting is saved.
   */
  const handleButtonClick = (remember: boolean) => saveRememberWindowSize(remember);

  /**
   * Handles the close event of the app window menu.
   * Sets the {@link isAppWindowMenuVisible} store to false.
   */
  const handleClose = () => isAppWindowMenuVisible.set(false);

  /**
   * Handles the keydown event of the app window menu.
   * If the key is "Escape", closes the menu.
   * @param {KeyboardEvent} event - The keydown event.
   */
  const handleKeydown = (event: KeyboardEvent) => {
    if (!$isAppWindowMenuVisible) return;

    if (event.key === "Escape") handleClose();
  };
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $isAppWindowMenuVisible}
  <!-- svelte-ignore a11y-no-static-element-interactions, a11y-click-events-have-key-events -->
  <div
    class="backdrop"
    on:click={handleClose}
    in:fade={{ duration: 100 }}
    out:fade={{ duration: 100 }}
  ></div>
  <div
    use:focusTrap
    class="menu-dialog"
    role="dialog"
    aria-modal="true"
    in:fly={{ y: 200, duration: 100 }}
    out:fade={{ duration: 100 }}
  >
    <div class="menu-content">
      <h1>{$t["app_window.title"]}</h1>
      <div class="option-buttons">
        <button
          bind:this={defaultButton}
          on:click={() => handleButtonClick(false)}
          class:active={!$appConfig.rememberWindowSize}
        >
          {$t["app_window.option.default"]}
        </button>

        <button
          bind:this={rememberButton}
          on:click={() => handleButtonClick(true)}
          class:active={$appConfig.rememberWindowSize}
        >
          {$t["app_window.option.remember"]}
        </button>
      </div>

      <button on:click={handleClose} class="close-button">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          height="1.25rem"
          viewBox="0 -960 960 960"
          width="1.25rem"
          fill="var(--color-close-button)"
          ><path
            d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"
          /></svg
        >
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
    display: flex;
    flex-direction: column;
    align-items: center;
    width: clamp(28ch, 32ch, 90vw);
    padding: 2.5rem;
    min-height: auto;
    background-color: var(--color-background);
    border: 0.2rem solid var(--color-outline);
    border-radius: 0.1rem;
    box-shadow: 0 2px 8px var(--color-shadow);
  }

  .menu-content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    align-items: center;
  }

  h1 {
    margin: 0 0 1rem 0;
    color: var(--color-text-primary);
    line-height: 1.2;
    font-size: 1.25rem;
    text-wrap: balance;
  }

  .option-buttons {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
    align-items: center;
  }

  button {
    width: fit-content;
    min-width: 100%;
    border: 0.15rem solid var(--color-outline);
    padding: 0.5rem 1rem;
    color: var(--color-text-primary);
    background-color: var(--color-button);
    border-radius: 0.1rem;
    box-shadow: 0.25rem 0.25rem 0 var(--color-outline);
    font-size: 0.9rem;
    font-weight: 600;
    transition:
      transform 150ms ease,
      box-shadow 150ms ease,
      background-color 150ms ease,
      color 150ms ease;
  }

  button:hover {
    transform: translate(0.15rem, 0.15rem);
    box-shadow: 0.1rem 0.1rem 0 var(--color-outline);
  }

  button:active {
    transform: translate(0.35rem, 0.35rem);
    box-shadow: 0 0 0 var(--color-outline);
  }

  button:focus {
    outline: none;
    background-color: var(--color-accent);
    color: var(--color-text-tertiary);
  }

  .close-button {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 1.5rem;
    font-size: 0.9rem;
    padding: 0.5rem 1rem;
    min-width: 0;
    color: var(--color-close-button);
  }
</style>
