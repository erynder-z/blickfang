<script lang="ts">
  import { isAppWindowMenuVisible, appConfig } from "$lib/stores/appState";
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

  const saveRememberWindowSize = async (remember: boolean) => {
    try {
      await invoke("update_remember_window_size_command", { remember });
    } catch (error) {
      console.error("Failed to save remember window size:", error);
    }
  };

  const handleButtonClick = (remember: boolean) => saveRememberWindowSize(remember);

  const handleClose = () => isAppWindowMenuVisible.set(false);

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

    display: flex;
    flex-direction: column;
    align-items: center;

    width: clamp(30ch, 35ch, 90vw);
    min-height: 25rem;
    padding: 1.5rem;

    background-color: var(--color-background);
    border: 1px solid var(--color-accent);
    border-radius: 8px;
    box-shadow: 0 4px 12px var(--color-shadow);

    transition: height 0.2s ease;
  }

  .menu-content {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    justify-content: center;
    gap: 1rem;
    text-align: center;
    width: 100%;
    max-width: 100%;
    overflow-wrap: break-word;
  }

  h1 {
    color: var(--color-text-primary);
    line-height: 1.2;
    text-wrap: balance;
    min-height: 2.5em;
    margin: 0;
  }

  button {
    padding: 0.5rem;
    border: solid 0.15rem var(--color-outline);
    border-radius: 0.5rem;
    background-color: transparent;
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: bold;

    min-height: 2.5rem;
    text-wrap: balance;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  button:focus {
    outline: solid 0.15rem var(--color-accent);
    outline-offset: 0.15rem;
  }

  button.active {
    background-color: var(--color-accent);
  }

  .close-button {
    margin-top: 1rem;
    background-color: var(--color-close-button);
  }
</style>
