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
  import { onMount } from "svelte";
  import { fly, fade } from "svelte/transition";
  import { focusTrap } from "$lib/actions/focusTrap";

  let defaultShortcuts: Shortcuts | null = null;

  onMount(async () => {
    defaultShortcuts = await invoke<Shortcuts>("get_default_shortcuts_command");
  });

  $: isUsingDefault =
    defaultShortcuts &&
    $appConfig.shortcuts &&
    JSON.stringify($appConfig.shortcuts) === JSON.stringify(defaultShortcuts);

  const handleRemap = () => {
    isRemapping.set(true);
  };

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

      <div class="toggle-buttons">
        <button class:active={isUsingDefault} on:click={handleSetDefault}>
          {$t["hotkeys.button.default_hotkeys"]}
        </button>
        <button class:active={!isUsingDefault} on:click={handleSetCustom}>
          {$t["hotkeys.button.custom_hotkeys"]}
        </button>
      </div>

      <div class="hotkeys-grid">
        {#if $appConfig.shortcuts}
          {#each Object.keys($appConfig.shortcuts) as action}
            <div class="hotkey-action">{$t[`hotkeys.${action}`]}</div>
            <div class="hotkey-key">
              {$appConfig.shortcuts[action as keyof Shortcuts].label.toLocaleUpperCase()}
            </div>
          {/each}
        {/if}
      </div>

      <button on:click={handleRemap} class="remap-button">{$t["hotkeys.button.remap"]}</button>

      <button on:click={handleClose} class="close-button">{$t["general.close"]}</button>
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
    width: clamp(40ch, 45ch, 90vw);
    min-height: 25rem;
    padding: 1.5rem;
    background-color: var(--color-background);
    border: 0.15rem solid var(--color-accent);
    border-radius: 0.5rem;
    box-shadow:
      0.3rem 0.3rem 0 0 var(--color-outline),
      0.6rem 0.6rem 0 0 var(--color-shadow);
    transition: height 0.2s ease;
  }

  .menu-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    min-width: 300px;
    text-align: center;
    width: 100%;
  }

  h1 {
    color: var(--color-text-primary);
    line-height: 1.2;
    min-height: 2.5em;
    margin: 0;
  }

  .toggle-buttons {
    display: grid;
    grid-template-columns: 1fr 1fr;
    border-radius: 0.5rem;
    overflow: hidden;
    box-shadow: 0.2rem 0.2rem 0 0 var(--color-outline);
  }

  .toggle-buttons button {
    padding: 0.5rem;
    border: none;
    background-color: var(--color-button);
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: bold;
    transition: all 0.15s ease;
    min-height: 2.5rem;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .toggle-buttons button:first-child {
    border-right: 0.15rem solid var(--color-accent);
  }

  .toggle-buttons button:hover {
    background-color: color-mix(in srgb, var(--color-button) 85%, var(--color-accent));
    transform: translateY(-1px);
    box-shadow: 0.2rem 0.2rem 0 0 var(--color-outline);
  }

  .toggle-buttons button:active {
    transform: translateY(1px);
    box-shadow: 0.1rem 0.1rem 0 0 var(--color-outline);
  }

  .toggle-buttons button.active {
    background-color: var(--color-accent);
  }

  .toggle-buttons button:focus-visible {
    outline: 0.15rem solid var(--color-accent);
    outline-offset: 0.15rem;
  }

  .hotkeys-grid {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 0.5rem 1.5rem;
    text-align: left;
  }

  .hotkey-action {
    color: var(--color-text-primary);
  }

  .hotkey-key {
    font-weight: bold;
    color: var(--color-text-secondary);
    justify-self: end;
  }

  .remap-button,
  .close-button {
    padding: 0.5rem;
    border: 0.15rem solid var(--color-outline);
    border-radius: 0.5rem;
    background-color: var(--color-button);
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: bold;
    min-height: 2.5rem;
    white-space: nowrap;
    text-overflow: ellipsis;
    transition: all 0.15s ease;
    box-shadow: 0.2rem 0.2rem 0 0 var(--color-outline);
  }

  .remap-button:hover,
  .close-button:hover {
    background-color: color-mix(in srgb, var(--color-button) 85%, var(--color-accent));
    transform: translateY(-1px);
    box-shadow: 0.3rem 0.3rem 0 0 var(--color-outline);
  }

  .remap-button:active,
  .close-button:active {
    transform: translateY(1px);
    box-shadow: 0.1rem 0.1rem 0 0 var(--color-outline);
  }

  .remap-button:focus,
  .close-button:focus {
    outline: 0.15rem solid var(--color-accent);
    outline-offset: 0.15rem;
  }

  .close-button {
    margin-top: 1rem;
    background-color: var(--color-close-button);
  }
</style>
