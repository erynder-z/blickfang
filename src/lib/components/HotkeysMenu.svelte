<script lang="ts">
  import {
    isHotkeysMenuVisible,
    appConfig,
    isRemapping,
    type Shortcuts,
  } from "$lib/stores/appState";
  import { t } from "$lib/utils/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import RemapDialog from "./RemapDialog.svelte";
  import { onMount, tick } from "svelte"; // Added tick import
  import { fly, fade } from "svelte/transition";
  import { focusTrap } from "$lib/actions/focusTrap";

  let defaultShortcuts: Shortcuts | null = null;
  let toggleButtons: HTMLButtonElement[] = [];

  onMount(async () => {
    defaultShortcuts = await invoke<Shortcuts>("get_default_shortcuts_command");
  });

  $: isUsingDefault =
    defaultShortcuts &&
    $appConfig.shortcuts &&
    JSON.stringify($appConfig.shortcuts) === JSON.stringify(defaultShortcuts);

  $: if ($isHotkeysMenuVisible)
    tick().then(() => {
      toggleButtons[0]?.focus();
    });

  const handleRemap = () => isRemapping.set(true);

  const handleClose = () => isHotkeysMenuVisible.set(false);

  const handleSetDefault = () => {
    invoke("set_active_shortcuts_to_default");
  };

  const handleSetCustom = () => {
    invoke("set_active_shortcuts_to_custom");
  };

  const handleKeydown = (event: KeyboardEvent) => {
    if (!$isHotkeysMenuVisible) return;
    if (event.key === "Escape") handleClose();
  };
</script>

<svelte:window on:keydown={handleKeydown} />

<RemapDialog />

{#if $isHotkeysMenuVisible}
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
      <h1>{$t["hotkeys.heading"]}</h1>

      <div class="toggle-row">
        <button
          bind:this={toggleButtons[0]}
          class="toggle-button"
          class:active={isUsingDefault}
          on:click={handleSetDefault}
        >
          {$t["hotkeys.button.default_hotkeys"]}
        </button>

        <button
          bind:this={toggleButtons[1]}
          class="toggle-button"
          class:active={!isUsingDefault}
          on:click={handleSetCustom}
        >
          {$t["hotkeys.button.custom_hotkeys"]}
        </button>
      </div>

      <div class="hotkeys-grid">
        {#if $appConfig.shortcuts}
          {#each Object.keys($appConfig.shortcuts) as action}
            <div class="action-row">
              <div class="hotkey-action">{$t[`hotkeys.${action}`]}</div>
              <div class="hotkey-key">
                {$appConfig.shortcuts[action as keyof Shortcuts].label.toLocaleUpperCase()}
              </div>
            </div>
          {/each}
        {/if}
      </div>

      <button on:click={handleRemap} class="remap-button">{$t["hotkeys.button.remap"]}</button>

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
    width: clamp(50ch, 55ch, 90vw);
    padding: 2.5rem;
    background-color: var(--color-background);
    border: 0.15rem solid var(--color-outline);
    border-radius: 0.1rem;
    box-shadow:
      0.3rem 0.3rem 0 0 var(--color-outline),
      0.6rem 0.6rem 0 0 var(--color-shadow);
  }

  .menu-content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    align-items: center;
    min-width: 100%;
  }

  h1 {
    margin: 0 0 1rem 0;
    color: var(--color-text-primary);
    line-height: 1.2;
    font-size: 1.25rem;
    text-wrap: balance;
  }

  .toggle-row {
    display: flex;
    gap: 0.75rem;
  }

  .toggle-button {
    border: 0.15rem solid var(--color-outline);
    padding: 0.5rem 1rem;
    border-radius: 0.1rem;
    color: var(--color-text-primary);
    background-color: var(--color-button);
    box-shadow: 0.25rem 0.25rem 0 var(--color-outline);
    font-size: 0.9rem;
    font-weight: 700;
    transition:
      transform 150ms ease,
      box-shadow 150ms ease,
      background-color 150ms ease,
      color 150ms ease;
  }

  .toggle-button:hover {
    transform: translate(0.15rem, 0.15rem);
    box-shadow: 0.1rem 0.1rem 0 var(--color-outline);
  }

  .toggle-button:active {
    transform: translate(0.35rem, 0.35rem);
    box-shadow: 0 0 0 var(--color-outline);
  }

  .toggle-button.active {
    background-color: var(--color-accent);
    color: var(--color-text-tertiary);
  }

  .toggle-button:focus-visible {
    outline: 0.15rem solid var(--color-accent);
    outline-offset: 0.2rem;
  }

  .hotkeys-grid {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
  }

  .action-row {
    display: flex;
    justify-content: space-between;
  }

  .hotkey-action {
    color: var(--color-text-primary);
  }

  .hotkey-key {
    text-align: end;
    font-weight: bold;
    color: var(--color-text-secondary);
  }

  .remap-button,
  .close-button {
    width: fit-content;
    min-width: 100%;
    border: 0.15rem solid var(--color-outline);
    padding: 0.5rem 1rem;
    border-radius: 0.1rem;
    color: var(--color-text-primary);
    background-color: var(--color-button);
    box-shadow: 0.25rem 0.25rem 0 var(--color-outline);
    font-size: 0.9rem;
    font-weight: 600;
    transition:
      transform 150ms ease,
      box-shadow 150ms ease,
      background-color 150ms ease,
      color 150ms ease;
  }

  .remap-button:hover,
  .close-button:hover {
    transform: translate(0.15rem, 0.15rem);
    box-shadow: 0.1rem 0.1rem 0 var(--color-outline);
  }

  .remap-button:active,
  .close-button:active {
    transform: translate(0.35rem, 0.35rem);
    box-shadow: 0 0 0 var(--color-outline);
  }

  .remap-button:focus,
  .close-button:focus {
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
